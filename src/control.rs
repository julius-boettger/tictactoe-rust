use crate::model::*;
use Status as S;

use crate::view;
use crate::constants::*;
use itertools::Itertools;

/// return the field that won a line (row/column/diagonal, if present)
fn get_line_winner(line: &[Field]) -> Field {
    let only_field_in_line: Result<&Field, _> = line.iter().all_equal_value();
    if let Ok(winner) = only_field_in_line {
        if winner.is_some() {
            return *winner;
        }
    }
    None
}

/// check if a line can not be won by anyone anymore (draw)
fn get_line_draw(line: &[Field]) -> bool {
    // a line is a draw if there are at least two unique fields on it (excluding Empty)
    line.iter()
        .unique()
        .filter(|&&field| field.is_some())
        .collect_vec()
        .len() >= 2
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
    for col_i in 0 .. BOARD_SIZE {
        let mut column: Line = [None; BOARD_SIZE];
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
    // diagonal_factor will be used to get the two possible diagonals
    for diagonal_factor in [0, BOARD_SIZE - 1] {
        // get diagonal from board
        let mut diagonal: Line = [None; BOARD_SIZE];
        // for all rows/columns
        for line in 0 .. BOARD_SIZE {
            // use diagonal factor to get major and minor diagonal values
            diagonal[line] = board[line][line.abs_diff(diagonal_factor)];
        }
        diagonals.push(diagonal)
    }
    diagonals
}

/// get all lines of the board (rows, columns and diagonals)
fn get_board_lines(board: &Board) -> Vec<Line> {
    let mut lines = Vec::new();
    lines.append(&mut get_rows(board));
    lines.append(&mut get_columns(board));
    lines.append(&mut get_diagonals(board));
    lines
}

/// get first winner of board
fn get_board_winner(board_lines: &Vec<Line>) -> Field {
    let winners = board_lines.iter()
        .map(|line| get_line_winner(line))
        .filter(|&option| option.is_some())
        .map(|option| option.unwrap())
        .collect_vec();

    if !winners.is_empty() {
        return Some(winners[0]);
    }

    None
}

/// check if board is a draw
fn get_board_draw(board_lines: &Vec<Line>) -> bool {
    // check the board for a draw (all lines have to be draws)
    let draw = board_lines.iter()
        .map(|line| get_line_draw(line))
        .all_equal_value();

    // if all lines have the same draw value and it is true
    if let Ok(value) = draw {
        if value {
            return true;
        }
    }

    false
}

/// get the current status of a board
pub fn get_board_status(board: &Board) -> Status {

    let board_lines = get_board_lines(board);

    if let Some(field) = get_board_winner(&board_lines) {
        return S::SomeoneWon(field);
    }

    if get_board_draw(&board_lines) {
        return S::Draw;
    }

    // the game is still going if neither a winner nor a draw could be determined
    Status::StillPlaying
}

/// construct a board. if content is `Some` it must be a vector with length `BOARD_SIZE.pow(2)`.
pub fn construct_board(content: Option<Vec<Field>>) -> Board {
    let mut board: Board = [[None; BOARD_SIZE]; BOARD_SIZE];

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

/// run a game of tic tac toe
pub fn run_game() {

    if BOARD_SIZE < 3 {
        panic!("BOARD_SIZE is {}, needs to be at least 3", BOARD_SIZE);
    }

    let board: Board = construct_board(Some(
        [Some('X'); BOARD_SIZE.pow(2)].to_vec()
    ));
    view::output::print_board_template();
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
                if board[row][col].is_some() {
                    panic!();
                }
            }
        }
        assert_eq!(get_board_status(&board), S::StillPlaying);
    }

    #[test]
    fn multiple_winners() {
        let mut board = construct_board(None);
        board[0] = [Some('X'); BOARD_SIZE];
        board[1] = [Some('X'); BOARD_SIZE];
        assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
    }

    #[test]
    fn winner_in_rows() {
        for winner_row in 0..BOARD_SIZE {
            let mut board = construct_board(None);
            board[winner_row] = [Some('X'); BOARD_SIZE];
            assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
        }
    }

    #[test]
    fn winner_in_cols() {
        for winner_col in 0..BOARD_SIZE {
            let mut board = construct_board(None);
            for row in 0..BOARD_SIZE {
                board[row][winner_col] = Some('X');
            }
            assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
        }
    }

    #[test]
    fn winner_in_diagonals() {
        for diagonal_factor in [0, BOARD_SIZE - 1] {
            let mut board = construct_board(None);
            for line in 0 .. BOARD_SIZE {
                board[line][line.abs_diff(diagonal_factor)] = Some('X');
            }
            assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
        }
    }
}