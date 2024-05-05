use std::sync::Mutex;
use std::collections::{HashSet, HashMap};
use rand::Rng;

pub const ROWS: usize = 9;
pub const COLS: usize = 9;
const EMPTY: usize = 0;
const BLACK: usize = 1;
const WHITE: usize = 2;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Intersection {
    Empty,
    Black(Group),
    White(Group),
}

pub struct Board {
    pub pieces: Mutex<Vec<Vec<Intersection>>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Group {
    pub intersections: HashSet<(usize, usize)>,
    pub liberties: HashSet<(usize, usize)>,
}

pub struct Hash {
    pub zobrist: Mutex<Zobrist>,
}

pub struct Zobrist {
    positions: [[[u64; 3]; COLS]; ROWS],
    board: Vec<Vec<Intersection>>,
    hash: u64,
    prev_hashes: HashSet<u64>,
}

impl Zobrist {
    pub fn new() -> Zobrist {
        // initialize keys for positions
        let mut positions = [[[0; 3]; COLS]; ROWS];
        let mut rng = rand::thread_rng();
        for i in 0..ROWS {
            for j in 0..COLS {
                for k in 0..3 {
                    positions[i][j][k] = rng.gen();
                }
            }
        }

        // calculate initial hash
        let mut hash: u64 = 0;
        let mut prev_hashes: HashSet<u64> = HashSet::new();
        for i in 0..ROWS {
            for j in 0..COLS {
                hash ^= positions[i][j][EMPTY];
            }
        }
        prev_hashes.insert(hash);
        let board: Vec<Vec<Intersection>> = vec![vec![Intersection::Empty; COLS]; ROWS];

        Zobrist {
            positions,
            board,
            hash,
            prev_hashes,
        }
    }

    pub fn update(&mut self, new_board: &Vec<Vec<Intersection>>) -> bool {
        let mut new_hash: u64 = self.hash;
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.board[i][j] == new_board[i][j] {
                    continue;
                }
                let old_color = match self.board[i][j] {
                    Intersection::Empty => EMPTY,
                    Intersection::Black(_) => BLACK,
                    Intersection::White(_) => WHITE,
                };
                let new_color = match new_board[i][j] {
                    Intersection::Empty => EMPTY,
                    Intersection::Black(_) => BLACK,
                    Intersection::White(_) => WHITE,
                };
                let old_key = self.positions[i][j][old_color];
                let new_key = self.positions[i][j][new_color];
                new_hash ^= old_key ^ new_key;
            }
        }

        if self.prev_hashes.contains(&new_hash) {
            false
        } else {
            self.hash = new_hash;
            self.board = new_board.clone();
            self.prev_hashes.insert(self.hash);
            true
        }
    }
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
pub fn get_liberties(x: usize, y: usize, color: usize, board: &mut Vec<Vec<Intersection>>) {
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
pub fn get_intersections(x: usize, y: usize, color: usize, board: &mut Vec<Vec<Intersection>>) {
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

// simulate a move to check for ko
pub fn simulate_move(x: usize, y: usize, color: usize, board: &Vec<Vec<Intersection>>, hash: &mut Zobrist) -> bool {
    // simulate the move
    let mut sim_board = board.clone();
    if color == 1 {
        sim_board[x][y] = Intersection::Black(Group { intersections: HashSet::new(), liberties: HashSet::new() });
    } else {
        sim_board[x][y] = Intersection::White(Group { intersections: HashSet::new(), liberties: HashSet::new() });
    }
    get_intersections(x, y, color, &mut sim_board);
    get_liberties(x, y, color, &mut sim_board);

    // get any pieces that need to be removed
    let mut to_remove: Vec<(usize, usize)> = vec![];
    let mut check_remove = |row: usize, col: usize| {
        if sim_board[row][col] == Intersection::Empty {
            return;
        }
        let actual_color = match color {
            1 => 2,
            2 => 1,
            _ => 0,
        };
        get_liberties(row, col, actual_color, &mut sim_board);
        match &sim_board[row][col] {
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
        sim_board[i.0][i.1] = Intersection::Empty;
    }

    // check for ko
    let is_ko = hash.update(&sim_board);
    is_ko
}

