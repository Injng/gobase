// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::collections::HashSet;

const ROWS: usize = 9;
const COLS: usize = 9;

#[derive(Debug, Clone)]
enum Intersection {
    Empty,
    Black(Group),
    White(Group),
}

struct Board {
    pieces: Mutex<Vec<Vec<Intersection>>>,
}

#[derive(Debug, Clone)]
struct Group {
    intersections: HashSet<(usize, usize)>,
    liberties: HashSet<(usize, usize)>,
}

// give coordinates, remove the liberties of adjacent groups
fn remove_liberties(x: usize, y: usize, color: usize, board: &mut Vec<Vec<Intersection>>) {
    if x > 0 {
        match &mut board[x - 1][y] {
            Intersection::Black(ref mut group) => {
                if color == 2 {
                    group.liberties.remove(&(x, y));
                }
            },
            Intersection::White(ref mut group) => {
                if color == 1 {
                    group.liberties.remove(&(x, y));
                }
            },
            _ => (),
        }
    }
    if x < ROWS - 1 {
        match &mut board[x + 1][y] {
            Intersection::Black(ref mut group) => {
                if color == 2 {
                    group.liberties.remove(&(x, y));
                }
            },
            Intersection::White(ref mut group) => {
                if color == 1 {
                    group.liberties.remove(&(x, y));
                }
            },
            _ => (),
        }
    }
    if y > 0 {
        match &mut board[x][y - 1] {
            Intersection::Black(ref mut group) => {
                if color == 2 {
                    group.liberties.remove(&(x, y));
                }
            },
            Intersection::White(ref mut group) => {
                if color == 1 {
                    group.liberties.remove(&(x, y));
                }
            },
            _ => (),
        }
    }
    if y < COLS - 1 {
        match &mut board[x][y + 1] {
            Intersection::Black(ref mut group) => {
                if color == 2 {
                    group.liberties.remove(&(x, y));
                }
            },
            Intersection::White(ref mut group) => {
                if color == 1 {
                    group.liberties.remove(&(x, y));
                }
            },
            _ => (),
        }
    }
}

// give coordinates, use flood fill to find all coordinates in the group
fn get_intersections(x: usize, y: usize, color: usize, board: &mut Vec<Vec<Intersection>>, group: &mut Group) {
    let mut visited = vec![vec![false; COLS]; ROWS];
    let mut queue = vec![(x, y)];
    while !queue.is_empty() {
        // get current intersection from queue
        let (x, y) = queue.pop().unwrap();
        if visited[x][y] {
            continue;
        }
        visited[x][y] = true;

        // update intersections and liberties according to color
        match board[x][y] {
            Intersection::Empty => {
                drop(group.liberties.insert((x, y)));
                continue;
            },
            Intersection::Black(_) => {
                if color == 1 {
                    group.intersections.insert((x, y));
                    remove_liberties(x, y, color, board);
                } else {
                    continue;
                }
            },
            Intersection::White(_) => {
                if color == 2 {
                    group.intersections.insert((x, y));
                    remove_liberties(x, y, color, board);
                } else {
                    continue;
                }
            },
        }

        // update queue for flood fill
        if x > 0 { queue.push((x - 1, y)); }
        if x < ROWS - 1 { queue.push((x + 1, y)); }
        if y > 0 { queue.push((x, y - 1)); }
        if y < COLS - 1 { queue.push((x, y + 1)); }
    }
}

// get constants
#[tauri::command]
fn get_rows() -> usize {
    ROWS
}

#[tauri::command]
fn get_cols() -> usize {
    COLS
}

// check if a given move is valid
#[tauri::command]
fn validate(x: usize, y: usize, color: usize, board: tauri::State<Board>) -> bool {
    let mut board = board.pieces.lock().unwrap();
    let intersection = &board[x][y];
    match intersection {
        Intersection::Empty => true,
        _ => false,
    }
}

// TODO: SUICIDE RULE
// handle a move by returning a list of intersections to remove
// precondition: move is valid
#[tauri::command]
fn handle_move(x: usize, y: usize, color: usize, board: tauri::State<Board>) -> Vec<(usize, usize)> {
    let mut board = board.pieces.lock().unwrap();

    // get and set intersections for the move
    let empty_group: Group = Group { intersections: HashSet::new(), liberties: HashSet::new() };
    let mut move_group: Group = Group { intersections: HashSet::new(), liberties: HashSet::new() };
    if color == 1 { 
        board[x][y] = Intersection::Black(empty_group); 
    } else { 
        board[x][y] = Intersection::White(empty_group); 
    }
    get_intersections(x, y, color, &mut board, &mut move_group);
    // print out all intersections of move_group
    println!("intersections: {:?}", move_group.intersections);
    println!("liberties: {:?}", move_group.liberties);

    if color == 1 { 
        board[x][y] = Intersection::Black(move_group); 
    } else { 
        board[x][y] = Intersection::White(move_group); 
    }

    // get any pieces that need to be removed
    let mut to_remove: Vec<(usize, usize)> = vec![];
    if x > 0 {
        match &board[x - 1][y] {
            Intersection::Black(group) => {
                if color == 2 && group.liberties.len() == 0 { 
                    board[x - 1][y] = Intersection::Empty;
                    to_remove.push((x - 1, y));
                }
            },
            Intersection::White(group) => {
                if color == 1 && group.liberties.len() == 0 {
                    board[x - 1][y] = Intersection::Empty;
                    to_remove.push((x - 1, y));
                }
            },
            _ => (),
        }
    }
    if x < ROWS - 1 {
        match &board[x + 1][y] {
            Intersection::Black(group) => {
                if color == 2 && group.liberties.len() == 0 {
                    board[x + 1][y] = Intersection::Empty;
                    to_remove.push((x + 1, y));
                }
            },
            Intersection::White(group) => {
                if color == 1 && group.liberties.len() == 0 {
                    board[x + 1][y] = Intersection::Empty;
                    to_remove.push((x + 1, y));
                }
            },
            _ => (),
        }
    }
    if y > 0 {
        match &board[x][y - 1] {
            Intersection::Black(group) => {
                if color == 2 && group.liberties.len() == 0 {
                    board[x][y - 1] = Intersection::Empty;
                    to_remove.push((x, y - 1));
                }
            },
            Intersection::White(group) => {
                if color == 1 && group.liberties.len() == 0 {
                    board[x][y - 1] = Intersection::Empty;
                    to_remove.push((x, y - 1));
                }
            },
            _ => (),
        }
    }
    if y < COLS - 1 {
        match &board[x][y + 1] {
            Intersection::Black(group) => {
                if color == 2 && group.liberties.len() == 0 {
                    board[x][y + 1] = Intersection::Empty;
                    to_remove.push((x, y + 1));
                }
            },
            Intersection::White(group) => {
                if color == 1 && group.liberties.len() == 0 {
                    board[x][y + 1] = Intersection::Empty;
                    to_remove.push((x, y + 1));
                }
            },
            _ => (),
        }
    }
    println!("to_remove: {:?}", to_remove);
    to_remove
}

fn main() {
    tauri::Builder::default()
        .manage(Board { pieces: Mutex::new(vec![vec![Intersection::Empty; 19]; 19])})
        .invoke_handler(tauri::generate_handler![get_rows, get_cols, validate, handle_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
