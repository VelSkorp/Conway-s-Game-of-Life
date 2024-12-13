use std::thread::sleep;
use std::time::Duration;

// Dimensions of the board
const WIDTH: usize = 50;
const HEIGHT: usize = 20;

// Different View Modes:
// 0: Monochrome characters ('O' for live, ' ' for dead)
// 1: Different ASCII characters ('@' for live, '.' for dead)
// 2: Use ANSI colors with characters
const VIEW_MODE: usize = 2;

fn main() {
    let mut board = initialize_board(WIDTH, HEIGHT);
    
    loop {
        print_board(&board);
        board = next_generation(&board);
        sleep(Duration::from_millis(100));
    }
}

fn initialize_board(width: usize, height: usize) -> Vec<bool> {
    let mut board = vec![false; width * height];
    let mid_col = width / 2;

    set_collumn(mid_col, vec![6, 7, 8, 9, 10, 11, 12, 13], &mut board);
    board
}

fn print_board(board: &[bool]) {
    // Clear screen and move cursor to top-left
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell_alive = board[row * WIDTH + col];
            match VIEW_MODE {
                0 => {
                    // Simple monochrome view
                    print!("{}", if cell_alive { "O" } else { " " });
                }
                1 => {
                    // Different ASCII characters
                    print!("{}", if cell_alive { "@" } else { "." });
                }
                2 => {
                    // ANSI colors: green for alive, dark gray for dead
                    // 32m = green; 90m = bright black (dark gray)
                    if cell_alive {
                        print!("\x1b[32mO\x1b[0m");
                    } else {
                        print!("\x1b[90m.\x1b[0m");
                    }
                }
                _ => {
                    // Default to the original mode if out of range
                    print!("{}", if cell_alive { "O" } else { " " });
                }
            }
        }
        println!();
    }
}

fn next_generation(board: &[bool]) -> Vec<bool> {
    let mut new_board = vec![false; WIDTH * HEIGHT];
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let idx = row * WIDTH + col;
            let count = live_neighbor_count(board, row, col);
            let cell = board[idx];
            new_board[idx] = match (cell, count) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    new_board
}

fn live_neighbor_count(board: &[bool], row: usize, col: usize) -> usize {
    let mut count = 0;
    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = (row as isize + dr).rem_euclid(HEIGHT as isize) as usize;
            let nc = (col as isize + dc).rem_euclid(WIDTH as isize) as usize;
            if board[nr * WIDTH + nc] {
                count += 1;
            }
        }
    }
    count
}

fn set_row(row: usize, collumns: Vec<usize>, board: &mut Vec<bool>) {
    for collumn in collumns {
        board[row * WIDTH + collumn] = true;
    }
}

fn set_collumn(collumn: usize, rows: Vec<usize>, board: &mut Vec<bool>) {
    for row in rows {
        board[row * WIDTH + collumn] = true;
    }
}
