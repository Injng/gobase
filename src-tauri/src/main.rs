// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

#[derive(Clone)]
enum Intersection {
    Empty,
    Black(Group),
    White(Group),
}

struct Board {
    pieces: Mutex<Vec<Vec<Intersection>>>,
}

#[derive(Clone)]
struct Group {
    intersections: Vec<(usize, usize)>,
    liberties: Vec<(usize, usize)>,
}

// check if a given move is valid
#[tauri::command]
fn validate(x: usize, y: usize, color: usize, board: tauri::State<Board>) -> bool {
    let mut board = board.pieces.lock().unwrap();
    let intersection = &board[x][y];
    let is_valid: bool;
    match intersection {
        Intersection::Empty => is_valid = true,
        _ => is_valid = false,
    }

    // handle whether or not move is valid
    if is_valid {
        if color == 1 { 
            board[x][y] = Intersection::Black(Group { intersections: vec![(x, y)], liberties: vec![] }); 
        } else { 
            board[x][y] = Intersection::White(Group { intersections: vec![(x, y)], liberties: vec![] }); 
        }
        return true;
    }
    return false;
}

fn main() {
    tauri::Builder::default()
        .manage(Board { pieces: Mutex::new(vec![vec![Intersection::Empty; 19]; 19])})
        .invoke_handler(tauri::generate_handler![validate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
