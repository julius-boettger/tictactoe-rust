use crate::model::Field;
use Field as F;
use crate::model::Status;
use Status as S;

use crate::view;
use crate::constants::*;
use itertools::Itertools;

/// get the current status of a board
pub fn get_board_status(board: &Board) -> Status {
    /// return the field that won a line (row/column/diagonal, if present)
    fn someone_won(line: &[Field]) -> Option<Field> {
        // check if the whole line consists of the same field
        let only_field_in_line: Result<&Field, _> = line.iter().all_equal_value();
        // if so and its not Empty, return it as the winner
        if let Ok(winner) = only_field_in_line {
            if *winner != F::Empty {
                return Some(*winner);
            }
        }
        // else return None
        None
    }

    /// check if a line can not be won by anyone anymore (draw)
    fn is_draw(line: &[Field]) -> bool {
        // a line is a draw if there are at least two unique fields on it (excluding Empty)
        line.iter()
            // unique fields
            .unique()
            // filter out empty fields
            .filter(|&&field| field != F::Empty)
            // return true if at least two unique fields
            .collect_vec().len() >= 2
    }

    /// returns rows of board
    fn get_rows(board: &Board) -> Vec<Line> {
        board.iter()
             .map(|&row| row)
             .collect_vec()
    }

    /// returns columns of board
    fn get_columns(board: &Board) -> Vec<Line> {
        let mut columns: Vec<Line> = Vec::new();
        // for all columns
        for col_i in 0 .. BOARD_SIZE {
            // get column from board
            let mut column: Line = [F::Empty; BOARD_SIZE];
            for row_i in 0 .. BOARD_SIZE {
                column[row_i] = board[row_i][col_i];
            }
            columns.push(column);
        }
        columns
    }

    /// returns diagonals of board
    fn get_diagonals(board: &Board) -> Vec<Line> {
        let mut diagonals: Vec<Line> = Vec::new();
        // for all diagonals
        // diagonal_factor will be used to get the two possible diagonals
        for diagonal_factor in [0, BOARD_SIZE - 1] {
            // get diagonal from board
            let mut diagonal: Line = [F::Empty; BOARD_SIZE];
            // for all rows/columns
            for line in 0 .. BOARD_SIZE {
                // use diagonal factor to get major and minor diagonal values
                diagonal[line] = board[line][line.abs_diff(diagonal_factor)];
            }
            diagonals.push(diagonal)
        }
        diagonals
    }

    // get all relevant arrays of fields of the board (rows, columns, diagonals)
    let mut relevant_board_lines = Vec::new();
    relevant_board_lines.append(&mut get_rows(board));
    relevant_board_lines.append(&mut get_columns(board));
    relevant_board_lines.append(&mut get_diagonals(board));

    // check all relevant board lines for winners
    let winners = relevant_board_lines.iter()
        // map to potential winner
        .map(|line| someone_won(line))
        // filter out None's
        .filter(|&option| option.is_some())
        // unwrap
        .map(|option| option.unwrap())
        .collect_vec();

    // return the first winner if there are winners
    if !winners.is_empty() {
        return S::SomeoneWon(winners[0]);
    }

    // check the board for a draw (all lines have to be draws)
    let draw = relevant_board_lines.iter()
        // map to draw in line (true or false)
        .map(|line| is_draw(line))
        // check if all values are equal
        .all_equal_value();

    // if all lines have the same draw value and it is true
    if let Ok(value) = draw {
        if value {
            return S::Draw;
        }
    }

    // the game is still going if neither a winner or a draw could be determined
    Status::StillPlaying
}

pub fn construct_board(content: Option<Vec<Field>>) -> Board {
    let mut board: Board = [[F::Empty; BOARD_SIZE]; BOARD_SIZE];

    let content = match content {
        Some(value) => value,
        None => return board
    };

    if content.len() == BOARD_SIZE.pow(2) {
        let mut index = 0;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                board[row][col] = content[index];
                index += 1;
            }
        }
    }

    board
}

pub fn run_game() {

    if BOARD_SIZE < 3 {
        panic!("BOARD_SIZE is {}, needs to be at least 3", BOARD_SIZE);
    }

    use Field::*;
    let board: Board = construct_board(Some(vec![
        X, O, O, O,
        X, O, O, X,
        X, O, O, O,
        X, O, X, O,
    ]));
    view::output::print_board(&board);
    println!("{}", get_board_status(&board));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_board() {
        let board = construct_board(None);
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if board[row][col] != F::Empty {
                    panic!();
                }
            }
        }
        assert_eq!(get_board_status(&board), S::StillPlaying);
    }

    #[test]
    fn multiple_winners() {
        let mut board = construct_board(None);
        board[0] = [F::X; BOARD_SIZE];
        board[1] = [F::O; BOARD_SIZE];
        assert_eq!(get_board_status(&board), S::SomeoneWon(F::X));
    }

    #[test]
    fn winner_in_rows() {
        for winner_row in 0..BOARD_SIZE {
            let mut board = construct_board(None);
            board[winner_row] = [F::X; BOARD_SIZE];
            assert_eq!(get_board_status(&board), S::SomeoneWon(F::X));
        }
    }

    #[test]
    fn winner_in_cols() {
        for winner_col in 0..BOARD_SIZE {
            let mut board = construct_board(None);
            for row in 0..BOARD_SIZE {
                board[row][winner_col] = F::X;
            }
            assert_eq!(get_board_status(&board), S::SomeoneWon(F::X));
        }
    }

    #[test]
    fn winner_in_diagonals() {
        for diagonal_factor in [0, BOARD_SIZE - 1] {
            let mut board = construct_board(None);
            for line in 0 .. BOARD_SIZE {
                board[line][line.abs_diff(diagonal_factor)] = F::X;
            }
            assert_eq!(get_board_status(&board), S::SomeoneWon(F::X));
        }
    }
}