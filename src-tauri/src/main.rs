// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod go;

use std::collections::HashSet;
use std::sync::Mutex;
use go::{get_intersections, get_liberties, simulate_move, Board, Group, Intersection, Hash, Zobrist, COLS, ROWS};

// get constants
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

// check if a given move is valid
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

    println!("suicide: {}", is_valid);

    // check for ko
    if is_valid {
        is_valid = simulate_move(x, y, color, &board, &mut hash);
    }

    println!("ko: {}", is_valid);

    is_valid
}

// handle a move by returning a list of intersections to remove
// precondition: move is valid
#[tauri::command]
fn handle_move(x: usize, y: usize, color: usize, board: tauri::State<Board>) -> Vec<(usize, usize)> {
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

    to_remove
}

fn main() {
    tauri::Builder::default()
        .manage(Board { pieces: Mutex::new(vec![vec![Intersection::Empty; COLS]; ROWS])})
        .manage(Hash { zobrist: Mutex::new(Zobrist::new()) })
        .invoke_handler(tauri::generate_handler![get_rows, get_cols, reset, validate, handle_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application"); 
}

