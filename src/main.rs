use std::io::{self, Read};
use std::error::Error;

mod cli;
mod controller;
mod game;

const BOARD_LEN: usize = 9;
const SQUARE_LEN: usize = 3;

fn main() {

    cli::enable_raw_mode();
    cli::enter_alternate_screen();

    let res: Result<(), Box<dyn Error>> = play();

    // Always clean up terminal mode first
    cli::leave_alternate_screen();
    cli::disable_raw_mode();

    // Now handle the result cleanly in standard terminal mode
    match res {
        Ok(()) => println!("You won!"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn play() -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J");

    let (term_width, term_height) = cli::terminal_size();

    // Sudoku box-drawing layout:
    //   width  = left border + 9 * (3-char cell + 1 divider)      = 1 + 9*4 = 37
    //   height = top border + 9 rows + 8 inner dividers + bottom  = 1 + 9 + 8 + 1 = 19
    let render_w = 1 + (BOARD_LEN * 4) as u16;
    let render_h = (1 + BOARD_LEN + (BOARD_LEN - 1) + 1) as u16;

    let start_col = term_width.saturating_sub(render_w) / 2;
    let start_row = term_height.saturating_sub(render_h) / 2;

    let mut cursor_x = 0usize;
    let mut cursor_y = 0usize;

    let init_grid = vec![ // matrix state: 0 = empty cell, or 1..=9 for a digit
        vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
        vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
        vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
        vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
        vec![4, 2, 6, 8, 0, 3, 7, 9, 1],
        vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
        vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
        vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
        vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
    ];

    let mut board = game::Board::new(init_grid, BOARD_LEN, SQUARE_LEN)?;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0u8; 3];

    loop {
        print!("\x1B[2J");
        board.draw_grid(start_col, start_row, cursor_x, cursor_y);

        let n = handle.read(&mut buf).unwrap();
        if n == 1 {
            let b = buf[0];
            let coords = game::Coordinates {
                row: cursor_y,
                col: cursor_x,
            };
            match b {
                b'1'..=b'9' => {
                    if board.givens[cursor_y][cursor_x] {
                        // can't overwrite a clue
                    } else {
                        let new_value = b - b'0'; // 1..=9

                        let is_valid_move = board.set_cell_if_valid_move(new_value, &coords);
                        if is_valid_move {
                            if game::is_win(&board) {
                                return Ok(());
                            }
                        }
                    }
                }
                b' ' | 0x7f => {
                    board.delete_cell_if_valid(&coords);
                }
                b'q' => return Ok(()), // TODO should return an exception when quitting?
                _ => {}
            }
        } else if n == 3 && buf[0] == 0x1B && buf[1] == b'[' {
            match buf[2] {
                b'A' => cursor_y = cursor_y.saturating_sub(1),
                b'B' => cursor_y = (cursor_y + 1).min(BOARD_LEN - 1),
                b'C' => cursor_x = (cursor_x + 1).min(BOARD_LEN - 1),
                b'D' => cursor_x = cursor_x.saturating_sub(1),
                _ => {}
            }
        }
    }
}
