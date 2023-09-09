use crate::constants::*;

/// print the board using the field display format and spacers
pub fn print_board(board: &Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            print!("{}  ", board[row][col]);
        }
        println!();
    }
}