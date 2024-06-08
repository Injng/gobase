use std::sync::{Arc, Mutex};
use crate::go::Intersection;

pub const ROWS: usize = 19;
pub const COLS: usize = 19;
const EMPTY: usize = 0;
const BLACK: usize = 1;
const WHITE: usize = 2;

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
}

pub struct Game {
    pub root: Arc<Mutex<Node>>,
    pub curr: Arc<Mutex<Node>>,
}

impl Game {
    /* TODO: parse sgf
    pub fn from_sgf(sgf: &str) {
        // strip first and last parantheses
        let mut loaded = sgf.chars();
        loaded.next();
        loaded.next();
        loaded.next_back();
        
        // get tokens
        let tokens = loaded.as_str().split(";");
    }
    */

    pub fn new() -> Game {
        let board = vec![vec![Intersection::Empty; COLS]; ROWS];
        let root = Arc::new(Mutex::new(Node::new(board, (0, 0), BLACK, None)));
        let curr = Arc::clone(&root);
        Game { root, curr }
    }

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
}

