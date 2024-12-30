use std::io::{self};
use std::sync::mpsc;
use std::thread;

/// Set up a channel to listen for user commands in a separate thread.
pub fn setup_command_listener() -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        let mut input = String::new();
        loop {
            input.clear();
            if let Ok(_) = stdin.read_line(&mut input) {
                let command = input.trim().to_string();
                match command.as_str() {
                    "pause" | "resume" | "faster" | "slower" | "toggle_wrap" | "step" => {
                        let _ = tx.send(command); // Send valid commands
                    }
                    _ => println!("Unknown command: {}", command),
                }
            }
        }
    });

    rx
}
