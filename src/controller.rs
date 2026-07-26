use std::io::{self, Write};

fn move_to(
    col: u16,
    row: u16,
) {
    print!("\x1B[{};{}H", row + 1, col + 1);
}

pub fn draw_grid(
    start_col: u16,
    start_row: u16,
    grid_w: usize,
    grid_h: usize,
    cursor_x: usize,
    cursor_y: usize,
    cells: &Vec<Vec<char>>,
) {
    for y in 0..grid_h {
        move_to(start_col, start_row + y as u16);
        let mut line = String::new();
        for x in 0..grid_w {
            let ch = cells[y][x];
            let is_cursor = x == cursor_x && y == cursor_y;

            if is_cursor {
                // yellow background highlight for the cursor cell
                line.push_str("\x1B[43m"); // yellow bg
                line.push('[');
                line.push(ch);
                line.push(']');
                line.push_str("\x1B[0m"); // reset formatting
            } else {
                line.push('[');
                line.push(ch);
                line.push(']');
            }
        }
        print!("{}", line);
    }
    io::stdout().flush().unwrap();
}
