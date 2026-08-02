pub struct Board {
    grid: Matrix<u8>,
    pub givens: Matrix<bool>,
    len: usize,
    square_len: usize,
    placed_cells_count: usize,
}

type Matrix<T> = Vec<Vec<T>>;

pub const UNSET_CELL: u8 = 0;

impl Board {
    pub fn new(
        init_grid: Matrix<u8>,
        len: usize,
        square_len: usize,
    ) -> Result<Self, String> {

        if len < 1 || len > 9 {
            return Err("Len must be between 1 and 9!".to_string());
        }

        Self::is_grid_valid(&init_grid, len, square_len)?;

        let givens = init_givens(&init_grid, len);

        let placed_cells_count = Self::init_placed_cells_count(&givens, len);

        return Ok(
            Self{
                grid: init_grid,
                givens,
                len,
                square_len,
                placed_cells_count,
            });
    }

    fn is_grid_valid(
        grid: &Matrix<u8>,
        len: usize,
        square_len: usize,
    ) -> Result<(), String> {
        for r in 0..len {
            for c in 0..len {
                let coords = Coordinates{row: r, col: c};
                let cell_value = grid[r][c];

                if cell_value != UNSET_CELL && cell_value < 1 || cell_value > (len as u8) {
                    return Err(format!(
                        "all values should be the unset value={UNSET_CELL}, or between 1 and len={len}, got {cell_value} at row={r}, col={c}"
                    ));
                }

                if !is_move_valid(grid, cell_value, &coords, square_len) {
                    return Err(format!(
                        "cell value {cell_value} at row={r}, col={c} is invalid"
                    ));
                }
            }
        }

        return Ok(());
    }

    fn init_placed_cells_count(
        givens: &Matrix<bool>,
        len: usize,
    ) -> usize {
        let mut placed_cells_count: usize = 0;
        for r in 0..len {
            for c in 0..len {
                placed_cells_count += givens[r][c] as usize;
            }
        }

        return placed_cells_count;
    }

    pub fn get_cell(
        &self,
        coords: &Coordinates
    ) -> u8 {
        return self.grid[coords.row][coords.col];
    }

    pub fn set_cell_if_valid_move(
        &mut self,
        new_value: u8,
        coords: &Coordinates,
    ) -> bool {
        if self.get_cell(coords) == new_value {
            return false;
        }

        if !is_move_valid(&self.grid, new_value as u8, &coords, self.square_len) {
            return false;
        }

        self.grid[coords.row][coords.col] = new_value;
        self.placed_cells_count += 1;
        return true;
    }

    pub fn delete_cell_if_valid(
        &mut self,
        coords: &Coordinates,
    ) -> bool {
        if !self.givens[coords.row][coords.col] && self.get_cell(coords) != UNSET_CELL {
            self.grid[coords.row][coords.col] = UNSET_CELL; // space or backspace clears the cell
            self.placed_cells_count -= 1;
            return true;
        }

        return false;
    }
}

fn init_givens(
    grid: &Matrix<u8>,
    len: usize
) -> Matrix<bool> {
    let mut givens = vec![vec![true; len]; len];

    for r in 0..len {
        for c in 0..len {
            if grid[r][c] == UNSET_CELL {
                givens[r][c] = false;
            }
        }
    }

    return givens
}

pub fn is_win(board: &Board) -> bool {
    return board.placed_cells_count == board.len*board.len;
}

pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

fn is_move_valid(
    grid: &Matrix<u8>,
    new_value: u8,
    coords: &Coordinates,
    square_len: usize,
) -> bool {
    if !is_row_valid(grid, new_value, coords) {
        return false;
    }

    if !is_col_valid(grid, new_value, coords) {
        return false;
    }

    if !is_square_valid(grid, square_len, new_value, coords) {
        return false;
    }

    return true;
}

pub fn is_row_valid(
    grid: &Matrix<u8>,
    new_value: u8,
    coords: &Coordinates,
) -> bool {
    for c in 0..8 {
        if c == coords.col {
            continue;
        }

        if grid[coords.row][c] == new_value {
            return false;
        }
    }

    return true;
}

pub fn is_col_valid(
    grid: &Matrix<u8>,
    new_value: u8,
    coords: &Coordinates,
) -> bool {
    for r in 0..8 {
        if r == coords.row {
            continue;
        }

        if grid[r][coords.col] == new_value {
            return false;
        }
    }

    return true;
}

pub fn is_square_valid(
    grid: &Matrix<u8>,
    square_len: usize,
    new_value: u8,
    coords: &Coordinates,
) -> bool {
    let square_col = coords.col / square_len; // integer floor divisions
    let square_row = coords.row / square_len;

    let offset_col = square_col * square_len;
    let offset_row = square_row * square_len;

    for r in offset_row..offset_row + square_len {
        for c in offset_col..offset_col + square_len {
            if r == coords.row && c == coords.col {
                continue;
            }

            if grid[r][c] == new_value {
                return false;
            }
        }
    }

    return true;
}
