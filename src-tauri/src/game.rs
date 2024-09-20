use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::go::{Intersection, Zobrist};

pub const ROWS: usize = 19;
pub const COLS: usize = 19;
pub const BLACK: usize = 1;
pub const WHITE: usize = 2;

pub enum Node {
    Move {
        board: Vec<Vec<Intersection>>,
        piece: (usize, usize),
        color: usize,
        parent: Option<Arc<Mutex<Node>>>,
        children: Vec<Arc<Mutex<Node>>>,
    },
    End {
        board: Vec<Vec<Intersection>>,
        piece: (usize, usize),
        color: usize,
        parent: Option<Arc<Mutex<Node>>>,
    },
}

impl Node {
    pub fn new(board: Vec<Vec<Intersection>>, piece: (usize, usize), color: usize, parent: Option<Arc<Mutex<Node>>>) -> Node {
        Node::Move {
            board,
            piece,
            color,
            parent,
            children: vec![],
        }
    }

    pub fn end(board: Vec<Vec<Intersection>>, piece: (usize, usize), color: usize, parent: Option<Arc<Mutex<Node>>>) -> Node {
        Node::End {
            board,
            piece,
            color,
            parent,
        }
    }

    pub fn get_board(&self) -> Vec<Vec<Intersection>> {
        match self {
            Node::Move { board, .. } => board.clone(),
            Node::End { board, .. } => board.clone(),
        }
    }
}

/// Struct to represent saved game
#[derive(Serialize, Deserialize)]
pub struct Saved {
    pub sgf: String,
    states: Vec<(Vec<Vec<Intersection>>, Zobrist)>
}

impl Saved {
    pub fn new(game: &Game) -> Saved {
        let sgf: String = game.to_sgf();
        let states: Vec<(Vec<Vec<Intersection>>, Zobrist)> = game.states.iter()
            .map(|(board, _, _, hash)| (board.clone(), hash.clone())).collect();
        Saved { sgf, states }
    }
}

pub struct Game {
    pub root: Arc<Mutex<Node>>,
    pub curr: Arc<Mutex<Node>>,
    pub states: Vec<(Vec<Vec<Intersection>>, Arc<Mutex<Node>>, Arc<Mutex<Node>>, Zobrist)>
}

impl Game {
    pub fn new() -> Game {
        let board = vec![vec![Intersection::Empty; COLS]; ROWS];
        let root = Arc::new(Mutex::new(Node::new(board, (0, 0), BLACK, None)));
        let curr = Arc::clone(&root);
        Game { root, curr, states: Vec::new() }
    }

    /// Add a node to the game tree
    pub fn add_node(&mut self, board: Vec<Vec<Intersection>>, piece: (usize, usize), color: usize) {
        let node = Arc::new(Mutex::new(Node::new(board, piece, color, Some(Arc::clone(&self.curr)))));
        {
            let mut self_node = self.curr.lock().unwrap();
            match &mut *self_node {
                Node::Move { children, .. } => {
                    children.push(Arc::clone(&node));
                },
                _ => (),
            }
        }
        self.curr = node;
    }

    /// Save the current state of the game
    pub fn save_state(&mut self, board: Vec<Vec<Intersection>>, hash: Zobrist) {
        self.states.push((board, Arc::clone(&self.curr), Arc::clone(&self.root), hash));
    }

    /// Add states from a Saved game to the current game
    pub fn add_states(&mut self, saved: Saved) {
        // pre-fill states vector with number of saved states
        for i in 0..saved.states.len() {
            self.states.push((saved.states[i].0.clone(), Arc::clone(&self.curr), Arc::clone(&self.root), saved.states[i].1.clone()));
        }

        // iterate through node tree, matching the board to each saved state board
        fn traverse_node(game: &mut Game, node: &Arc<Mutex<Node>>, states: &Vec<Vec<Vec<Intersection>>>) {
            let mut node_data = node.lock().unwrap();
            match &mut *node_data {
                Node::Move { board, children, .. } => {
                    // search in states for matching board
                    for i in 0..states.len() {
                        if board == &states[i] {
                            game.states[i].1 = Arc::clone(node);
                            break;
                        }
                    }
                    for child in children {
                        traverse_node(game, child, states);
                    }
                },
                _ => (),
            }
        }
        
        let saved_states = saved.states.iter().map(|(board, _)| board.clone()).collect(); 
        let root_clone = Arc::clone(&self.root);
        traverse_node(self, &root_clone, &saved_states);
    }

    /// Convert the game tree to SGF string
    pub fn to_sgf(&self) -> String {
        let mut sgf = String::from("(;FF[4]GM[1]SZ[19]");

        fn traverse_node(node: &Arc<Mutex<Node>>, sgf: &mut String, mut first: usize) {
            let node = node.lock().unwrap();
            match &*node {
                Node::Move { piece, color, children, .. } => {
                    // ignore first placeholder node
                    if first < 1 {
                        first += 1;
                        if children.len() > 1 {
                            for child in children {
                                sgf.push('(');
                                traverse_node(child, sgf, first);
                                sgf.push(')');
                            }
                        } else if children.len() == 1 {
                            traverse_node(&children[0], sgf, first);
                        }
                        return;
                    }

                    // get move color and location
                    let color_str = if *color == BLACK { "B" } else { "W" };
                    let y = (piece.1 as u8 + b'a') as char;
                    let x = (piece.0 as u8 + b'a') as char;
                    sgf.push_str(&format!(";{}[{}{}]", color_str, x, y));
                    
                    // go to next node
                    if children.len() > 1 {
                        for child in children {
                            sgf.push('(');
                            traverse_node(child, sgf, first);
                            sgf.push(')');
                        }
                    } else if children.len() == 1 {
                        traverse_node(&children[0], sgf, first);
                    } else {
                        sgf.push(';');
                    }
                },
                Node::End { piece, color, .. } => {
                    println!("Node::End");
                    let color_str = if *color == BLACK { "B" } else { "W" };
                    let y = (piece.1 as u8 + b'a') as char;
                    let x = (piece.0 as u8 + b'a') as char;
                    sgf.push_str(&format!(";{}[{}{}]", color_str, x, y));
                },
            }
        }

        traverse_node(&self.root, &mut sgf, 0);
        sgf.push(')');
        sgf
    }
}

