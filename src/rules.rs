pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

pub fn is_move_valid(
    cells: &Vec<Vec<u8>>,
    new_char: u8,
    coords: Coordinates,
) -> bool {
    if !is_row_valid(cells, new_char, coords.row) {
        return false;
    }

    if !is_col_valid(cells, new_char, coords.col) {
        return false;
    }

    if !is_square_valid(cells, 3, new_char, coords) {
        return false;
    }

    return true;
}

pub fn is_row_valid(
    cells: &Vec<Vec<u8>>,
    new_char: u8,
    row: usize,
) -> bool {
    for c in 0..8 {
        if cells[row][c] == new_char {
            return false;
        }
    }

    return true;
}

pub fn is_col_valid(
    cells: &Vec<Vec<u8>>,
    new_char: u8,
    col: usize,
) -> bool {
    for r in 0..8 {
        if cells[r][col] == new_char {
            return false;
        }
    }

    return true;
}

pub fn is_square_valid(
    cells: &Vec<Vec<u8>>,
    square_len: usize,
    new_char: u8,
    coords: Coordinates,
) -> bool {
    let square_col = coords.col / square_len; // integer floor divisions
    let square_row = coords.row / square_len;

    let offset_col = square_col * square_len;
    let offset_row = square_row * square_len;

    for r in offset_row..offset_row + square_len {
        for c in offset_col..offset_col + square_len {
            if cells[r][c] == new_char {
                return false;
            }
        }
    }

    return true;
}

pub fn is_win(
    cells_count: usize,
    placed_chars: usize,
) -> bool {
    return placed_chars == cells_count;
}
