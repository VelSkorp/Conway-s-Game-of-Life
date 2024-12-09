use std::thread::sleep;
use std::time::Duration;

// Dimensions of the board
const WIDTH: usize = 50;
const HEIGHT: usize = 20;

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
    // Example: a simple blinker pattern in the center
    let mid_row = height / 2;
    let mid_col = width / 2;
    board[mid_row * width + mid_col - 1] = true;
    board[mid_row * width + mid_col] = true;
    board[mid_row * width + mid_col + 1] = true;
    board
}

fn print_board(board: &[bool]) {
    // Clear screen and move cursor to top-left
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell = board[row * WIDTH + col];
            print!("{}", if cell { "O" } else { " " });
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
