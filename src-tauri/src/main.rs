// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::collections::HashSet;

const ROWS: usize = 9;
const COLS: usize = 9;

#[derive(Debug, Clone, PartialEq)]
enum Intersection {
    Empty,
    Black(Group),
    White(Group),
}

struct Board {
    pieces: Mutex<Vec<Vec<Intersection>>>,
}

#[derive(Debug, Clone, PartialEq)]
struct Group {
    intersections: HashSet<(usize, usize)>,
    liberties: HashSet<(usize, usize)>,
}

// find if a intersection has any adjacent liberties and add them to the group
fn find_liberties(x: usize, y: usize, board: &Vec<Vec<Intersection>>, libs: &mut HashSet<(usize, usize)>) {
    let intersection = &board[x][y];
    match intersection {
        Intersection::Black(_) | Intersection::White(_) => {
            if x > 0 && board[x - 1][y] == Intersection::Empty {
                libs.insert((x - 1, y));
            }
            if x < ROWS - 1 && board[x + 1][y] == Intersection::Empty {
                libs.insert((x + 1, y));
            }
            if y > 0 && board[x][y - 1] == Intersection::Empty {
                libs.insert((x, y - 1));
            }
            if y < COLS - 1 && board[x][y + 1] == Intersection::Empty {
                libs.insert((x, y + 1));
            }
        }
        _ => {}
    }
}

// precondition: all intersections have been updated
fn get_liberties(x: usize, y: usize, color: usize, board: &mut Vec<Vec<Intersection>>) {
    // initialize clean group with no liberties
    let mut move_group: Group = Group { intersections: HashSet::new(), liberties: HashSet::new() };
    if let Intersection::Black(ref mut group) = board[x][y] {
        move_group.intersections = group.intersections.clone();
    } else if let Intersection::White(ref mut group) = board[x][y] {
        move_group.intersections = group.intersections.clone();
    } else {
        return;
    }

    // find liberties for all intersections in the group
    for i in move_group.intersections.iter() {
        find_liberties(i.0, i.1, board, &mut move_group.liberties);
    }

    // set move_group for all intersections in the group
    for i in move_group.intersections.iter() {
        if color == 1 {
            if let Intersection::Black(ref mut group) = board[i.0][i.1] {
                group.liberties = move_group.liberties.clone();
            }
        } else {
            if let Intersection::White(ref mut group) = board[i.0][i.1] {
                group.liberties = move_group.liberties.clone();
            }
        }
    }
}

// give coordinates, use flood fill to find all coordinates in the group
fn get_intersections(x: usize, y: usize, color: usize, board: &mut Vec<Vec<Intersection>>) {
    // initialize group with empty intersections and no liberties
    let mut group: Group = Group { intersections: HashSet::new(), liberties: HashSet::new() };

    // use flood fill to find all intersections in the group
    let mut visited = vec![vec![false; COLS]; ROWS];
    let mut queue = vec![(x, y)];
    while !queue.is_empty() {
        // get current intersection from queue
        let (x, y) = queue.pop().unwrap();
        if visited[x][y] {
            continue;
        }
        visited[x][y] = true;

        // update intersections according to color
        match board[x][y] {
            Intersection::Black(_) => {
                if color == 1 {
                    group.intersections.insert((x, y));
                } else {
                    continue;
                }
            },
            Intersection::White(_) => {
                if color == 2 {
                    group.intersections.insert((x, y));
                } else {
                    continue;
                }
            },
            _ => continue,
        }

        // update queue for flood fill
        if x > 0 { queue.push((x - 1, y)); }
        if x < ROWS - 1 { queue.push((x + 1, y)); }
        if y > 0 { queue.push((x, y - 1)); }
        if y < COLS - 1 { queue.push((x, y + 1)); }
    }

    // update for all intersections in the group
    for i in group.intersections.iter() {
        if color == 1 {
            if let Intersection::Black(ref mut ref_group) = board[i.0][i.1] {
                ref_group.intersections = group.intersections.clone();
            }
        } else {
            if let Intersection::White(ref mut ref_group) = board[i.0][i.1] {
                ref_group.intersections = group.intersections.clone();
            }
        }
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
        .manage(Board { pieces: Mutex::new(vec![vec![Intersection::Empty; 19]; 19])})
        .invoke_handler(tauri::generate_handler![get_rows, get_cols, validate, handle_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

