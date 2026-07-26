use std::io::{self, Read};

mod cli;
mod controller;
mod rules;

fn main() {
    cli::enable_raw_mode();
    cli::enter_alternate_screen();
    print!("\x1B[2J");

    let (term_width, term_height) = cli::terminal_size();
    let grid_w = 9;
    let grid_h = 9;
    let cell_len = 3;

    let start_col = term_width.saturating_sub((grid_w * cell_len) as u16) / 2;
    let start_row = term_height.saturating_sub(grid_h as u16) / 2;

    let mut cursor_x = 0usize;
    let mut cursor_y = 0usize;

    // grid state: ' ' = empty cell, or a digit character '0'..'9'
    let mut cells = vec![vec![' '; grid_w]; grid_h];

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0u8; 3];

    loop {
        print!("\x1B[2J");
        controller::draw_grid(
            start_col, start_row, grid_w, grid_h, cursor_x, cursor_y, &cells,
        );

        let n = handle.read(&mut buf).unwrap();

        if n == 1 {
            let b = buf[0];
            match b {
                b'w' => cursor_y = cursor_y.saturating_sub(1),
                b's' => cursor_y = (cursor_y + 1).min(grid_h - 1),
                b'a' => cursor_x = cursor_x.saturating_sub(1),
                b'd' => cursor_x = (cursor_x + 1).min(grid_w - 1),
                b'1'..=b'9' => {
                    if rules::is_move_valid(&cells, b, (cursor_x, cursor_y)) {
                        cells[cursor_y][cursor_x] = b as char; // if valid, place the digit
                    }
                }
                b' ' | 0x7f => {
                    cells[cursor_y][cursor_x] = ' '; // space or backspace clears the cell
                }
                b'q' => break,
                _ => {}
            }
        } else if n == 3 && buf[0] == 0x1B && buf[1] == b'[' {
            match buf[2] {
                b'A' => cursor_y = cursor_y.saturating_sub(1),
                b'B' => cursor_y = (cursor_y + 1).min(grid_h - 1),
                b'C' => cursor_x = (cursor_x + 1).min(grid_w - 1),
                b'D' => cursor_x = cursor_x.saturating_sub(1),
                _ => {}
            }
        }
    }

    cli::leave_alternate_screen();
    cli::disable_raw_mode();
}
