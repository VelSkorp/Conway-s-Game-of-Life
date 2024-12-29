use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::consts::{HEIGHT, WIDTH};
use crate::utils::cell_representation;

/// Initialize the board based on the given pattern or loaded file.
pub fn initialize_board(pattern: String, load_file: Option<String>) -> Vec<bool> {
    if let Some(filename) = load_file {
        load_board(&filename).unwrap_or_else(|e| {
            eprintln!("Failed to load board: {}", e);
            initialize_cross(WIDTH, HEIGHT)
        })
    } else {
        match pattern.as_str() {
            "glider" => initialize_glider(WIDTH, HEIGHT),
            "random" => initialize_random(WIDTH, HEIGHT),
            _ => initialize_cross(WIDTH, HEIGHT),
        }
    }
}

/// Initialize the board with a vertical line of live cells in the center column.
pub fn initialize_cross(width: usize, height: usize) -> Vec<bool> {
    let mut board = vec![false; width * height];
    let mid_col = width / 2;
    let mid_row = height / 2;
    set_column(mid_col, vec![6, 7, 8, 9, 10, 11, 12, 13], &mut board);
    set_row(mid_row, vec![6, 7, 8, 9, 10, 11, 12, 13], &mut board);
    board
}

/// Initialize the board with a glider pattern in the top-left corner.
pub fn initialize_glider(width: usize, height: usize) -> Vec<bool> {
    let mut board = vec![false; width * height];
    if width > 2 && height > 2 {
        // Glider cells
        board[idx(0, 1)] = true;
        board[idx(1, 2)] = true;
        board[idx(2, 0)] = true;
        board[idx(2, 1)] = true;
        board[idx(2, 2)] = true;
    }
    board
}

/// Initialize the board with random live and dead cells.
pub fn initialize_random(width: usize, height: usize) -> Vec<bool> {
    let mut board = vec![false; width * height];
    let mut rng = rand::thread_rng();
    for cell in &mut board {
        *cell = rng.gen_bool(0.2);
    }
    board
}

/// Prints the current state of the board to the terminal.
pub fn print_board(board: &[bool]) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell_alive = board[idx(row, col)];
            print!("{}", cell_representation(cell_alive));
        }
        println!();
    }
}

/// Save the board state to a file.
pub fn save_board(board: &[bool], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let cell = if board[idx(row, col)] { '1' } else { '0' };
            write!(file, "{}", cell)?;
        }
        writeln!(file)?;
    }
    Ok(())
}

/// Load the board state from a file.
pub fn load_board(filename: &str) -> io::Result<Vec<bool>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut board = vec![false; WIDTH * HEIGHT];
    for (row, line) in reader.lines().enumerate() {
        if row >= HEIGHT {
            break;
        }
        let line = line?;
        for (col, char) in line.chars().enumerate() {
            if col >= WIDTH {
                break;
            }
            board[idx(row, col)] = char == '1';
        }
    }
    Ok(board)
}

/// Calculates the 1D index from a 2D (row, column) pair.
#[inline]
pub fn idx(row: usize, col: usize) -> usize {
    row * WIDTH + col
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
