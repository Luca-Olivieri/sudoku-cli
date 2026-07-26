pub fn is_move_valid(
    cells: &Vec<Vec<char>>,
    new_char: u8,
    coordinates: (usize, usize),
) -> bool {
    let (x, y) = coordinates;
    for col in 0..8 {
        if cells[y][col] == new_char as char {
            return false;
        }
    }

    for row in 0..8 {
        if cells[row][x] == new_char as char {
            return false;
        }
    }

    return true;
}
