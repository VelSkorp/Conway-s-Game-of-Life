use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::env;
use rand::Rng;

// Dimensions of the board
const WIDTH: usize = 50;
const HEIGHT: usize = 20;

// Conway's Game of Life: B3/S23 (Birth on 3 neighbors, survive on 2 or 3)
const BIRTH: [usize; 1] = [3];
const SURVIVE: [usize; 2] = [2, 3];

// Different View Modes:
// 0: Monochrome characters ('O' for live, ' ' for dead)
// 1: Different ASCII characters ('@' for live, '.' for dead)
// 2: Use ANSI colors with characters
const VIEW_MODE: usize = 2;

fn main() {
    // Default pattern
    let mut pattern = "line".to_string();
    let mut load_file = None;

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--pattern" if i + 1 < args.len() => {
                pattern = args[i + 1].clone();
                i += 2;
            }
            "--load" if i + 1 < args.len() => {
                load_file = Some(args[i + 1].clone());
                i += 2;
            }
            _ => {
                // Ignore unknown arguments
                i += 1;
            }
        }
    }

    let mut board = if let Some(filename) = load_file {
        load_board(&filename).unwrap_or_else(|e| {
            eprintln!("Failed to load board: {}", e);
            initialize_line(WIDTH, HEIGHT)
        })
    } else {
        match pattern.as_str() {
            "glider" => initialize_glider(WIDTH, HEIGHT),
            "random" => initialize_random(WIDTH, HEIGHT),
            _ => initialize_line(WIDTH, HEIGHT),
        }
    };

    let mut generation = 0;

    loop {
        print_generation(generation);
        print_board(&board);

        if generation % 10 == 0 {
            save_board(&board, "game_of_life_state.txt").unwrap_or_else(|e| {
                eprintln!("Failed to save board: {}", e);
            });
        }

        board = next_generation(&board);
        generation += 1;
    }
}

/// Initialize the board with a vertical line of live cells in the center column.
fn initialize_line(width: usize, height: usize) -> Vec<bool> {
    let mut board = vec![false; width * height];
    let mid_col = width / 2;
    set_column(mid_col, vec![6, 7, 8, 9, 10, 11, 12, 13], &mut board);
    board
}

/// Initialize the board with a glider pattern in the top-left corner.
fn initialize_glider(width: usize, height: usize) -> Vec<bool> {
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
fn initialize_random(width: usize, height: usize) -> Vec<bool> {
    let mut board = vec![false; width * height];
    let mut rng = rand::thread_rng();
    for cell in &mut board {
        *cell = rng.gen_bool(0.2);
    }
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

            let should_live = if current_cell {
                SURVIVE.contains(&neighbors)
            } else {
                BIRTH.contains(&neighbors)
            };

            new_board[idx(row, col)] = should_live;
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

/// Save the board state to a file.
fn save_board(board: &[bool], filename: &str) -> io::Result<()> {
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
fn load_board(filename: &str) -> io::Result<Vec<bool>> {
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