use std::io::{self, Read};

use crate::rules::is_win;
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

    // Sudoku box-drawing layout:
    //   width  = left border + 9 * (3-char cell + 1 divider)      = 1 + 9*4 = 37
    //   height = top border + 9 rows + 8 inner dividers + bottom  = 1 + 9 + 8 + 1 = 19
    let render_w = 1 + (grid_w * 4) as u16;
    let render_h = (1 + grid_h + (grid_h - 1) + 1) as u16;

    let start_col = term_width.saturating_sub(render_w) / 2;
    let start_row = term_height.saturating_sub(render_h) / 2;

    let mut cursor_x = 0usize;
    let mut cursor_y = 0usize;

    // grid state: 0 = empty cell, or 1..=9 for a digit
    let mut cells = vec![vec![0u8; grid_w]; grid_h];
    // true where the cell was part of the original puzzle (rendered bold, not editable)
    let givens = vec![vec![false; grid_w]; grid_h]; // load a real puzzle here if you have one

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0u8; 3];

    let mut placed_chars: usize = 0;

    loop {
        print!("\x1B[2J");
        controller::draw_grid(start_col, start_row, &cells, &givens, cursor_x, cursor_y);

        let n = handle.read(&mut buf).unwrap();
        if n == 1 {
            let b = buf[0];
            match b {
                b'1'..=b'9' => {
                    if givens[cursor_y][cursor_x] {
                        // can't overwrite a clue
                    } else {
                        let digit = b - b'0'; // 1..=9
                        let coords = rules::Coordinates {
                            row: cursor_y,
                            col: cursor_x,
                        };
                        if rules::is_move_valid(&cells, digit, coords) {
                            cells[cursor_y][cursor_x] = digit;
                            placed_chars += 1;
                            if is_win(grid_w * grid_h, placed_chars) {
                                break;
                            }
                        }
                    }
                }
                b' ' | 0x7f => {
                    if !givens[cursor_y][cursor_x] && cells[cursor_y][cursor_x] != 0 {
                        cells[cursor_y][cursor_x] = 0; // space or backspace clears the cell
                        placed_chars -= 1;
                    }
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

    print!("You won!")
}
