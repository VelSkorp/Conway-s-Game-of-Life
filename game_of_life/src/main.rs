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
    let mut generation = 0;

    loop {
        print_generation(generation);
        print_board(&board);
        board = next_generation(&board);
        generation += 1;
        sleep(Duration::from_millis(100));
    }
}

/// Initializes the board with a vertical line of live cells in the center column.
fn initialize_board(width: usize, height: usize) -> Vec<bool> {
    let mut board = vec![false; width * height];
    let mid_col = width / 2;

    set_column(mid_col, vec![6, 7, 8, 9, 10, 11, 12, 13], &mut board);
    board
}

/// Prints the current state of the board to the terminal.
fn print_board(board: &[bool]) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell_alive = board[idx(row, col)];
            print!("{}", cell_representation(cell_alive));
        }
        println!();
    }
}

/// Computes the next generation of the board using Conway's Game of Life rules.
fn next_generation(board: &[bool]) -> Vec<bool> {
    let mut new_board = vec![false; WIDTH * HEIGHT];

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let neighbors = live_neighbor_count(board, row, col);
            let current_cell = board[idx(row, col)];

            new_board[idx(row, col)] = match (current_cell, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_board
}

/// Counts the number of live neighbors around a specific cell, wrapping around edges.
fn live_neighbor_count(board: &[bool], row: usize, col: usize) -> usize {
    let mut count = 0;

    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }

            let neighbor_row = ((row as isize + dr).rem_euclid(HEIGHT as isize)) as usize;
            let neighbor_col = ((col as isize + dc).rem_euclid(WIDTH as isize)) as usize;

            if board[idx(neighbor_row, neighbor_col)] {
                count += 1;
            }
        }
    }

    count
}

/// Sets specified columns in a particular row to true.
fn set_row(row: usize, columns: Vec<usize>, board: &mut Vec<bool>) {
    for column in columns {
        board[idx(row, column)] = true;
    }
}

/// Sets specified rows in a particular column to true.
fn set_column(column: usize, rows: Vec<usize>, board: &mut Vec<bool>) {
    for row in rows {
        board[idx(row, column)] = true;
    }
}

/// Returns a character representing the cell based on the selected VIEW_MODE.
fn cell_representation(cell_alive: bool) -> String {
    match VIEW_MODE {
        0 => {
            // Simple monochrome view
            if cell_alive { "O".to_string() } else { " ".to_string() }
        }
        1 => {
            // Different ASCII characters
            if cell_alive { "@".to_string() } else { ".".to_string() }
        }
        2 => {
            // ANSI colors: green for alive ('O'), dark gray for dead ('.')
            if cell_alive {
                "\x1b[32mO\x1b[0m".to_string()
            } else {
                "\x1b[90m.\x1b[0m".to_string()
            }
        }
        _ => {
            // Default to monochrome if out of range
            if cell_alive { "O".to_string() } else { " ".to_string() }
        }
    }
}

/// Clears the terminal screen and moves the cursor to the top-left corner.
fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

/// Calculates the 1D index from a 2D (row, column) pair.
#[inline]
fn idx(row: usize, col: usize) -> usize {
    row * WIDTH + col
}

fn print_generation(gen: usize) {
    clear_screen();
    println!("Generation: {}", gen);
}
