use std::io::{self, Write};

fn move_to(
    col: u16,
    row: u16,
) {
    print!("\x1B[{};{}H", row + 1, col + 1);
}

const N: usize = 9;

/// Builds a horizontal border line.
/// `fill` is the 3-char segment repeated under each cell (e.g. "---" or "===").
/// `thin_x`/`thick_x` are the intersection chars used every column / every 3rd column.
fn border(
    fill_thin: &str,
    fill_thick: &str,
    corner: char,
    thin_x: char,
    thick_x: char,
) -> String {
    let mut s = String::new();
    s.push(corner);
    for x in 0..N {
        let is_box_edge = (x + 1) % 3 == 0;
        s.push_str(if is_box_edge { fill_thick } else { fill_thin });
        if x == N - 1 {
            s.push(corner);
        } else if is_box_edge {
            s.push(thick_x);
        } else {
            s.push(thin_x);
        }
    }
    s
}

/// cells:  9x9 grid, 0 = empty, 1-9 = digit
/// givens: 9x9 grid, true = original puzzle clue (rendered bold)
pub fn draw_grid(
    start_col: u16,
    start_row: u16,
    cells: &Vec<Vec<u8>>,
    givens: &Vec<Vec<bool>>,
    cursor_x: usize,
    cursor_y: usize,
) {
    // Outer border and every 3rd row/col use thick ('=' / '#'), others use thin ('-' / '|')
    let top = border("===", "===", '#', '#', '#');
    let midlt = border("---", "---", '+', '|', '+'); // thin row divider
    let midbig = border("===", "===", '#', '#', '#'); // thick row divider (every 3rd row)
    let bottom = border("===", "===", '#', '#', '#');

    let mut row = start_row;
    move_to(start_col, row);
    print!("{}", top);
    row += 1;

    for y in 0..N {
        move_to(start_col, row);
        let mut line = String::new();
        line.push('#'); // thick outer left edge

        for x in 0..N {
            let val = cells[y][x];
            let ch = if val == 0 { ' ' } else { (b'0' + val) as char };
            let is_cursor = x == cursor_x && y == cursor_y;

            if is_cursor {
                line.push_str("\x1B[43m\x1B[30m"); // yellow bg, black fg
                line.push(' ');
                line.push(ch);
                line.push(' ');
                line.push_str("\x1B[0m");
            } else if givens[y][x] {
                line.push_str("\x1B[1m "); // bold given
                line.push(ch);
                line.push_str(" \x1B[0m");
            } else {
                line.push(' ');
                line.push(ch);
                line.push(' ');
            }

            // vertical divider: thick '#' every 3rd col (and outer edge), thin '|' otherwise
            if x == N - 1 {
                line.push('#'); // thick outer right edge
            } else if (x + 1) % 3 == 0 {
                line.push('#');
            } else {
                line.push('|');
            }
        }

        print!("{}", line);
        row += 1;

        // horizontal divider: thick every 3 rows, thin otherwise
        move_to(start_col, row);
        if y == N - 1 {
            print!("{}", bottom);
        } else if (y + 1) % 3 == 0 {
            print!("{}", midbig);
            row += 1;
        } else {
            print!("{}", midlt);
            row += 1;
        }
    }

    io::stdout().flush().unwrap();
}
