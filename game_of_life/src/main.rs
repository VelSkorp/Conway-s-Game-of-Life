mod args;
mod board;
mod commands;
mod consts;
mod simulation;
mod utils;

use crate::args::parse_arguments;
use crate::board::{initialize_board, print_board};
use crate::commands::setup_command_listener;
use crate::simulation::run_simulation;
use crate::utils::{disable_alternate_buffer, enable_alternate_buffer};

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
