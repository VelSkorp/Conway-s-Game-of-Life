use crate::board::{idx, print_board, save_board};
use crate::consts::{BIRTH, HEIGHT, SURVIVE, WIDTH, WRAPAROUND};
use crate::utils::print_generation;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Run the simulation loop, handling pause and resume commands.
pub fn run_simulation(
    mut board: Vec<bool>,
    rx: mpsc::Receiver<String>,
    running: Arc<AtomicBool>,
) -> Vec<bool> {
    let mut next_board = board.clone();
    let mut generation = 0;
    let mut paused = false;
    let mut delay = 200; // Initial delay in milliseconds

    while running.load(Ordering::SeqCst) {
        if let Ok(command) = rx.try_recv() {
            match command.as_str() {
                "pause" => paused = true,
                "resume" => paused = false,
                "faster" => {
                    if delay > 50 {
                        delay -= 50;
                        println!("Speed increased: Delay = {} ms", delay);
                    }
                }
                "slower" => {
                    delay += 50;
                    println!("Speed decreased: Delay = {} ms", delay);
                }
                "toggle_wrap" => {
                    let current = WRAPAROUND.load(Ordering::SeqCst);
                    WRAPAROUND.store(!current, Ordering::SeqCst);
                    println!(
                        "Wraparound is now {}",
                        if !current { "enabled" } else { "disabled" }
                    );
                }
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

        thread::sleep(Duration::from_millis(delay as u64)); // Delay for readability
    }

    board
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
    let wrap = WRAPAROUND.load(Ordering::SeqCst);

    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }

            let neighbor_row = if wrap {
                ((row as isize + dr).rem_euclid(HEIGHT as isize)) as usize
            } else {
                let new_row = row as isize + dr;
                if new_row < 0 || new_row >= HEIGHT as isize {
                    continue;
                }
                new_row as usize
            };

            let neighbor_col = if wrap {
                ((col as isize + dc).rem_euclid(WIDTH as isize)) as usize
            } else {
                let new_col = col as isize + dc;
                if new_col < 0 || new_col >= WIDTH as isize {
                    continue;
                }
                new_col as usize
            };

            if board[idx(neighbor_row, neighbor_col)] {
                count += 1;
            }
        }
    }

    count
}
