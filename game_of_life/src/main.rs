use ctrlc;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Dimensions of the board
const WIDTH: usize = 80;
const HEIGHT: usize = 30;

// Conway's Game of Life: B3/S23 (Birth on 3 neighbors, survive on 2 or 3)
const BIRTH: [usize; 1] = [3];
const SURVIVE: [usize; 2] = [2, 3];

// Different View Modes:
// 0: Monochrome characters ('O' for live, ' ' for dead)
// 1: Different ASCII characters ('@' for live, '.' for dead)
// 2: Use ANSI colors with characters
const VIEW_MODE: usize = 2;

fn main() {
    let (pattern, load_file, history) = parse_arguments();
    let board = initialize_board(pattern, load_file);
    let rx = setup_command_listener(); // Only the receiver is returned
    let running = Arc::new(AtomicBool::new(true));

    // Set Ctrl + C signal handler
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if !history {
        enable_alternate_buffer();
    }

    let last_board = run_simulation(board, rx, running);

    if !history {
        disable_alternate_buffer();
        println!("Final Generation:");
        print_board(&last_board);
    }
}

/// Parse command-line arguments and return the pattern and optional load file.
fn parse_arguments() -> (String, Option<String>, bool) {
    let mut pattern = "line".to_string();
    let mut load_file = None;
    let mut history = false;

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
            "--history" => {
                history = true;
                i += 1;
            }
            _ => {
                i += 1; // Ignore unknown arguments
            }
        }
    }
    (pattern, load_file, history)
}

/// Initialize the board based on the given pattern or loaded file.
fn initialize_board(pattern: String, load_file: Option<String>) -> Vec<bool> {
    if let Some(filename) = load_file {
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
    }
}

/// Set up a channel to listen for user commands in a separate thread.
fn setup_command_listener() -> mpsc::Receiver<&'static str> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        let mut input = String::new();
        loop {
            input.clear();
            if let Ok(_) = stdin.read_line(&mut input) {
                let command = input.trim();
                if command == "pause" {
                    let _ = tx.send("pause"); // Safely ignore send errors
                } else if command == "resume" {
                    let _ = tx.send("resume"); // Safely ignore send errors
                }
            }
        }
    });

    rx
}

/// Run the simulation loop, handling pause and resume commands.
fn run_simulation(
    mut board: Vec<bool>,
    rx: mpsc::Receiver<&'static str>,
    running: Arc<AtomicBool>,
) -> Vec<bool> {
    let mut next_board = board.clone();
    let mut generation = 0;
    let mut paused = false;

    while running.load(Ordering::SeqCst) {
        if let Ok(command) = rx.try_recv() {
            match command {
                "pause" => paused = true,
                "resume" => paused = false,
                _ => (),
            }
        }

        if paused {
            println!("Simulation paused. Type 'resume' to continue.");
            thread::sleep(Duration::from_secs(120));
            continue;
        }

        print_generation(generation);
        print_board(&board);

        if generation % 10 == 0 {
            save_board(&board, "game_of_life_state.txt").unwrap_or_else(|e| {
                eprintln!("Failed to save board: {}", e);
            });
        }

        next_generation(&mut board, &mut next_board);

        // Swap the buffers
        std::mem::swap(&mut board, &mut next_board);

        generation += 1;

        thread::sleep(Duration::from_millis(100)); // Delay for readability
    }

    board
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
fn next_generation(current: &Vec<bool>, next: &mut Vec<bool>) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let neighbors = live_neighbor_count(current, row, col);
            let current_cell = current[idx(row, col)];

            let should_live = if current_cell {
                SURVIVE.contains(&neighbors)
            } else {
                BIRTH.contains(&neighbors)
            };

            next[idx(row, col)] = should_live;
        }
    }
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
            if cell_alive {
                "O".to_string()
            } else {
                " ".to_string()
            }
        }
        1 => {
            // Different ASCII characters
            if cell_alive {
                "@".to_string()
            } else {
                ".".to_string()
            }
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
            if cell_alive {
                "O".to_string()
            } else {
                " ".to_string()
            }
        }
    }
}

fn enable_alternate_buffer() {
    print!("\x1b[?1049h");
    io::stdout().flush().unwrap();
}

fn disable_alternate_buffer() {
    print!("\x1b[?1049l");
    io::stdout().flush().unwrap();
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
