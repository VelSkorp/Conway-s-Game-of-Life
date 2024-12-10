# Conway's Game of Life in Rust (Console Version)

This project is a simple, terminal-based implementation of Conway's Game of Life written in Rust. It displays a 2D grid of cells, where each cell either "lives" or "dies" based on the number of its neighboring cells that are alive. By iterating through generations and applying these simple rules, fascinating emergent patterns often appear.

## Features

- **Terminal-Based Visualization:**  
  Renders the simulation state directly to your terminal, using characters to represent live and dead cells.
  
- **Minimal Dependencies:**  
  Uses only the Rust standard library for the core logic and `std::thread::sleep` for timing. No external crates are strictly required for a basic run.
  
- **Configurable Board and Patterns:**  
  Start with a simple pattern (e.g., a blinker) and modify the code to try out other initial configurations. You can also randomize the initial state by integrating the `rand` crate.

## Getting Started

### Prerequisites

- **Rust and Cargo:**  
  Ensure you have the Rust toolchain installed. If not, follow the instructions at [https://rustup.rs/](https://rustup.rs/).

### Building and Running

1. **Clone this repository:**
   ```bash
   git clone https://github.com/yourusername/rust-game-of-life.git
   cd rust-game-of-life
    ```

2. **Build the project:**
    ```bash
    cargo build --release
    ```

3. **Run the game:**
    ```bash
    cargo run --release
    ```

    You should see the board rendered to your terminal. The simulation will run indefinitely, updating every 100 milliseconds.

## Customizing the Simulation

- **Board Dimensions:**
Modify the constants `WIDTH` and `HEIGHT` in `main.rs` to change the size of the board.

- **Initial Pattern:**
The function `initialize_board()` sets up the initial pattern. By default, it places a "blinker" in the center. You can experiment with various patterns, such as gliders, or load a random initial state by adding the `rand` crate and calling `rand::random()` for each cell.

- **Ruleset and Behavior:**
The rules are currently hard-coded to the standard Conway's Life rules. You can modify the `next_generation()` function to experiment with different cellular automaton rules.

- **Refresh Rate:**
Currently, the simulation sleeps for 100 milliseconds between generations. Adjust the `std::thread::sleep(Duration::from_millis(100))` call in the main loop to speed up or slow down the simulation.

## Terminal Compatibility

Most modern terminals support the ANSI escape codes used for clearing and re-positioning the cursor. If you experience issues with display, consider using a crate like `crossterm` or `termion` for more reliable terminal control.

## Roadmap

- **Add User Interaction:**
Pause, resume, or speed up the simulation via keyboard input.

- **Different View Modes:**
Experiment with different characters or colors for living and dead cells.

- **Save and Load Patterns:**
Extend the program to read initial states from files, allowing you to load complex patterns.

## Contributing

Contributions are welcome! Please submit issues or pull requests with any improvements, bug fixes, or new features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.