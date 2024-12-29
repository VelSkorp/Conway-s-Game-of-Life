use std::io::{self, Write};

use crate::consts::VIEW_MODE;

pub fn enable_alternate_buffer() {
    print!("\x1b[?1049h");
    io::stdout().flush().unwrap();
}

pub fn disable_alternate_buffer() {
    print!("\x1b[?1049l");
    io::stdout().flush().unwrap();
}

/// Clears the terminal screen and moves the cursor to the top-left corner.
pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

pub fn print_generation(gen: usize) {
    clear_screen();
    println!("Generation: {}", gen);
}

/// Returns a character representing the cell based on the selected VIEW_MODE.
pub fn cell_representation(cell_alive: bool) -> String {
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
