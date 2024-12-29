// Dimensions of the board
pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 30;

// Conway's Game of Life: B3/S23 (Birth on 3 neighbors, survive on 2 or 3)
pub const BIRTH: [usize; 1] = [3];
pub const SURVIVE: [usize; 2] = [2, 3];

// Different View Modes:
// 0: Monochrome characters ('O' for live, ' ' for dead)
// 1: Different ASCII characters ('@' for live, '.' for dead)
// 2: Use ANSI colors with characters
pub const VIEW_MODE: usize = 2;
