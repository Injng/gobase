// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod go;
pub mod game;

use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, Mutex};
use go::{get_intersections, get_liberties, simulate_ko, Board, Group, Tree, Intersection, Hash, Zobrist, COLS, ROWS};
use game::{Saved, Game, Node, BLACK, WHITE};

#[tauri::command]
fn get_rows() -> usize {
    ROWS
}

#[tauri::command]
fn get_cols() -> usize {
    COLS
}

#[tauri::command]
fn reset(board: tauri::State<Board>, hash: tauri::State<Hash>) {
    let mut board = board.pieces.lock().unwrap();
    let mut hash = hash.zobrist.lock().unwrap();
    *board = vec![vec![Intersection::Empty; COLS]; ROWS];
    *hash = Zobrist::new();
}

/// Check if a given move is valid
#[tauri::command]
fn validate(x: usize, y: usize, color: usize, board: tauri::State<Board>, hash: tauri::State<Hash>) -> bool {
    let mut board = board.pieces.lock().unwrap();
    let mut hash = hash.zobrist.lock().unwrap();
    let intersection = &board[x][y];
    let mut is_valid: bool;

    // check for existing piece
    match intersection {
        Intersection::Empty => is_valid = true,
        _ => is_valid = false,
    }
    if !is_valid { return is_valid; }

    // prevent suicide by checking if group has any liberties
    is_valid = false;
    let mut check_liberty = |row: usize, col: usize| {
        if board[row][col] == Intersection::Empty {
            is_valid = true;
        } else if matches!(board[row][col], Intersection::Black(_)) && color == 1 {
            get_liberties(row, col, color, &mut board);
            if let Intersection::Black(group) = &board[row][col] {
                if group.liberties.len() > 1 {
                    is_valid = true;
                }
            }
        } else if matches!(board[row][col], Intersection::White(_)) && color == 2 {
            get_liberties(row, col, color, &mut board);
            if let Intersection::White(group) = &board[row][col] {
                if group.liberties.len() > 1 {
                    is_valid = true;
                }
            }
        }
    };
    if x > 0 {
        check_liberty(x - 1, y);
    }
    if x < ROWS - 1 {
        check_liberty(x + 1, y);
    }
    if y > 0 {
        check_liberty(x, y - 1);
    }
    if y < COLS - 1 {
        check_liberty(x, y + 1);
    }

    // except if it results in the capture of another group
    let mut check_capture = |row, col| {
        let actual_color = match color {
            1 => 2,
            2 => 1,
            _ => 0,
        };
        get_liberties(row, col, actual_color, &mut board);
        match &board[row][col] {
            Intersection::Black(group) => {
                if actual_color == 1 && group.liberties.len() == 1 { 
                    is_valid = true;
                }
            },
            Intersection::White(group) => {
                if actual_color == 2 && group.liberties.len() == 1 {
                    is_valid = true;
                }
            },
            _ => (),
        }
    };
    if x > 0 {
        check_capture(x - 1, y);
    }
    if x < ROWS - 1 {
        check_capture(x + 1, y);
    }
    if y > 0 {
        check_capture(x, y - 1);
    }
    if y < COLS - 1 {
        check_capture(x, y + 1);
    }

    // check for ko
    if is_valid {
        is_valid = simulate_ko(x, y, color, &board, &mut hash);
    }

    is_valid
}

/// Wrapper function for tauri to handle a move
#[tauri::command]
fn tauri_move(x: usize, y: usize, color: usize, board: tauri::State<Board>, tree: tauri::State<Tree>) -> Vec<(usize, usize)> {
    let piece: Vec<(usize, usize)> = handle_move(x, y, color, &board, &tree);
    return piece;
}

// precondition: move is valid
/// Handle a move by returning a list of intersections to remove
fn handle_move(x: usize, y: usize, color: usize, board: &tauri::State<Board>, tree: &tauri::State<Tree>) -> Vec<(usize, usize)> {
    let mut board = board.pieces.lock().unwrap();
    if color == 1 {
        board[x][y] = Intersection::Black(Group { intersections: HashSet::new(), liberties: HashSet::new() });
    } else {
        board[x][y] = Intersection::White(Group { intersections: HashSet::new(), liberties: HashSet::new() });
    }

    // update intersections and liberties for the move
    get_intersections(x, y, color, &mut board);
    get_liberties(x, y, color, &mut board);

    // get any pieces that need to be removed
    let mut to_remove: Vec<(usize, usize)> = vec![];
    let mut check_remove = |row: usize, col: usize| {
        if board[row][col] == Intersection::Empty {
            return;
        }
        let actual_color = match color {
            1 => 2,
            2 => 1,
            _ => 0,
        };
        get_liberties(row, col, actual_color, &mut board);
        match &board[row][col] {
            Intersection::Black(group) => {
                if actual_color == 1 && group.liberties.len() == 0 { 
                    to_remove.extend(group.intersections.iter());
                }
            },
            Intersection::White(group) => {
                if actual_color == 2 && group.liberties.len() == 0 {
                    to_remove.extend(group.intersections.iter());
                }
            },
            _ => (),
        }
    };
    if x > 0 {
        check_remove(x - 1, y);
    }
    if x < ROWS - 1 {
        check_remove(x + 1, y);
    }
    if y > 0 {
        check_remove(x, y - 1);
    }
    if y < COLS - 1 {
        check_remove(x, y + 1);
    }

    // update the board to make removed intersections empty
    for i in to_remove.iter() {
        board[i.0][i.1] = Intersection::Empty;
    }

    // update the game nodes
    let mut game = tree.game.lock().unwrap();
    game.add_node(board.clone(), (x, y), color);

    to_remove
}

/// Handle an undo move, and return (added_pieces, removed_pieces)
#[tauri::command]
fn handle_undo(board: tauri::State<Board>, tree: tauri::State<Tree>) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize)>) {
    let mut change_board = board.pieces.lock().unwrap();
    let mut game = tree.game.lock().unwrap();

    // update board to the parent node of current node
    let mut added_pieces = vec![];
    let mut removed_pieces = vec![];
    let mut parent_node = Arc::clone(&game.curr);
    {
        let curr = game.curr.lock().unwrap();
        if let Node::Move { parent, .. } = &*curr {
            if let Some(parent) = parent {
                parent_node = Arc::clone(&parent);
                let parent = parent.lock().unwrap();
                match &*parent {
                    Node::Move { board, .. } => {
                        // get differences between the boards
                        for i in 0..ROWS {
                            for j in 0..COLS {
                                if board[i][j] != change_board[i][j] {
                                    if board[i][j] == Intersection::Empty {
                                        removed_pieces.push((i, j));
                                    } else {
                                        added_pieces.push((i, j, match &board[i][j] {
                                            Intersection::Black(_) => 1,
                                            Intersection::White(_) => 2,
                                            _ => 0,
                                        }));
                                    }
                                }
                            }
                        }
                        // update the actual board
                        *change_board = board.clone();
                    },
                    _ => (),
                }
            }
        }
    }
    game.curr = Arc::clone(&parent_node);

    (added_pieces, removed_pieces)
}

/// Handle a redo move, and return (added_pieces, removed_pieces)
#[tauri::command]
fn handle_redo(board: tauri::State<Board>, tree: tauri::State<Tree>) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize)>) {
    let mut change_board = board.pieces.lock().unwrap();
    let mut game = tree.game.lock().unwrap();

    // update board to the last child node of current node
    // TODO: allow user to select variation of child node
    let mut added_pieces = Vec::new();
    let mut removed_pieces = Vec::new();
    let mut child_node = Arc::clone(&game.curr);
    {
        let curr = game.curr.lock().unwrap();
        if let Node::Move { children, .. } = &*curr {
            // see if there are any children to redo
            if children.len() == 0 {
                return (added_pieces, removed_pieces);
            }
            child_node = Arc::clone(&children[children.len() - 1]);
            let children = children[children.len() - 1].lock().unwrap();
            match &*children {
                Node::Move { board, .. } => {
                    // get differences between the boards
                    for i in 0..ROWS {
                        for j in 0..COLS {
                            if board[i][j] != change_board[i][j] {
                                if board[i][j] == Intersection::Empty {
                                    removed_pieces.push((i, j));
                                } else {
                                    added_pieces.push((i, j, match &board[i][j] {
                                        Intersection::Black(_) => BLACK,
                                        Intersection::White(_) => WHITE,
                                        _ => 0,
                                    }));
                                }
                            }
                        }
                    }
                    // update the actual board
                    *change_board = board.clone();
                },
                _ => (),
            }
        }
    }
    game.curr = Arc::clone(&child_node);

    (added_pieces, removed_pieces)
}
 
/// Evaluate a token from SGF
fn eval_token(token: &str, board: &tauri::State<Board>, tree: &tauri::State<Tree>) {
    // get action by retrieving characters before open bracket
    let action = token.chars()
        .take_while(|&c| c != '[')
        .collect::<String>();

    // if action is not "B" or "W", return early
    if action != "B" && action != "W" {
        return;
    }

    // otherwise, get coordinates by retrieving characters between brackets
    let coords = token.chars()
        .skip_while(|&c| c != '[').skip(1)
        .take_while(|&c| c != ']')
        .collect::<String>();

    // if coordinates are not two characters long, panic and error
    if coords.len() != 2 {
        panic!("Invalid coordinates in SGF: {}", coords);
    }

    // transform SGF coordinates to board coordinates
    let x = coords.chars().nth(0).unwrap() as usize - 'a' as usize;
    let y = coords.chars().nth(1).unwrap() as usize - 'a' as usize;

    // handle the move
    let color = if action == "B" { BLACK } else { WHITE };
    handle_move(x, y, color, board, tree);
}

/// Tauri wrapper function for creating a Game from a file containing an SGF string
#[tauri::command]
fn from_sgf_file(file: &str, board: tauri::State<Board>, tree: tauri::State<Tree>) -> Vec<(usize, usize, usize)> {
    // load sgf from file and create a new Game
    let sgf: &str = &fs::read_to_string(file).expect("Error: cannot read file");
    {
        let mut tree = tree.game.lock().unwrap();
        *tree = Game::new();
    }

    // clear the board
    {
        let mut board = board.pieces.lock().unwrap();
        *board = vec![vec![Intersection::Empty; COLS]; ROWS];
    }

    from_sgf(sgf, &board, &tree)
}

/// Create a Game from an SGF string, and return the added pieces
#[tauri::command]
fn from_sgf(sgf: &str, board: &tauri::State<Board>, tree: &tauri::State<Tree>) -> Vec<(usize, usize, usize)> {
    // strip first and last parantheses
    let mut loaded = sgf.chars();
    loaded.next();
    loaded.next();
    loaded.next_back();

    // get tokens
    let tokens = loaded.as_str().split(";");

    // evaluate tokens
    for token in tokens {
        eval_token(token, &board, &tree);
    }

    // iterate through board and add pieces
    let mut added: Vec<(usize, usize, usize)> = Vec::new();
    let board = board.pieces.lock().unwrap();
    for i in 0..ROWS {
        for j in 0..COLS {
            if let Intersection::Black(_) = board[i][j] {
                added.push((i, j, BLACK));
            } else if let Intersection::White(_) = board[i][j] {
                added.push((i, j, WHITE));
            }
        }
    }

    added
}

/// Saves the current state of the board
#[tauri::command]
fn save_state(board: tauri::State<Board>, tree: tauri::State<Tree>) {
    let board = board.pieces.lock().unwrap();
    let mut game = tree.game.lock().unwrap();
    game.save_state(board.clone());
}

/// Reverts to a saved state of the board
#[tauri::command]
fn revert_state(state_idx: usize, board: tauri::State<Board>, tree: tauri::State<Tree>) -> Vec<(usize, usize, usize)> {
    let mut board = board.pieces.lock().unwrap();
    let mut game = tree.game.lock().unwrap();
    if game.states.len() < state_idx + 1 {
        return Vec::new();
    }
    let (state, curr, root) = game.states[state_idx].clone();
    *board = state.to_vec();
    game.curr = curr.clone();
    game.root = root.clone();

    // iterate through board and add pieces
    let mut added: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            if let Intersection::Black(_) = board[i][j] {
                added.push((i, j, BLACK));
            } else if let Intersection::White(_) = board[i][j] {
                added.push((i, j, WHITE));
            }
        }
    }

    added
}

/// Initialize a number of states for frontend by returning number of states in backend
#[tauri::command]
fn init_states(tree: tauri::State<Tree>) -> usize {
    let game = tree.game.lock().unwrap();
    game.states.len()
}

/// Save the current nodes by saving to a SGF file
#[tauri::command]
fn save_sgf(file: &str, tree: tauri::State<Tree>) {
    // check if file ends in extension .sgf, and if not append
    let file = if file.ends_with(".sgf") { file.to_string() } else { format!("{}.sgf", file) };

    // save SGF file
    let game = tree.game.lock().unwrap();
    let sgf = game.to_sgf();
    fs::write(file, sgf).expect("Error: cannot write file");
}

/// Save the current Game by serializing it to JSON
#[tauri::command]
fn save_game(file: &str, tree: tauri::State<Tree>) {
    // check if file ends in extension .save, and if not append
    let file = if file.ends_with(".save") { file.to_string() } else { format!("{}.save", file) };

    // serialize Game into JSON and save
    let game = tree.game.lock().unwrap();
    let saved_game: Saved = Saved::new(&game);
    let saved_json: String = serde_json::to_string(&saved_game).unwrap();
    fs::write(file, saved_json).expect("Error: cannot write file");
}

/// Load a Game from a file containing a serialized Saved struct
#[tauri::command]
fn load_game(file: &str, board: tauri::State<Board>, tree: tauri::State<Tree>) -> Vec<(usize, usize, usize)> {
    // deserialize Saved struct from file
    let saved_json: &str = &fs::read_to_string(file).expect("Error: cannot read file");
    let saved_game: Saved = serde_json::from_str(saved_json).unwrap();

    // clear the board
    {
        let mut board = board.pieces.lock().unwrap();
        *board = vec![vec![Intersection::Empty; COLS]; ROWS];
    }

    // import SGF into game
    {
        let mut tree = tree.game.lock().unwrap();
        *tree = Game::new();
    }
    let added: Vec<(usize, usize, usize)> = from_sgf(&saved_game.sgf, &board, &tree);

    // import saved states into game
    let mut game = tree.game.lock().unwrap();
    game.add_states(saved_game);

    added
}

fn main() {
    tauri::Builder::default()
        .manage(Board { pieces: Mutex::new(vec![vec![Intersection::Empty; COLS]; ROWS])})
        .manage(Hash { zobrist: Mutex::new(Zobrist::new()) })
        .manage(Tree { game: Mutex::new(Game::new()) })
        .invoke_handler(tauri::generate_handler![get_rows, get_cols, reset, validate, 
            tauri_move, handle_undo, handle_redo, from_sgf_file, save_state, revert_state, 
            init_states, save_game, load_game, save_sgf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application"); 
}

