/* TODO's
 * - give expect("") real messages
 * - row.iter.collect_vec() could be better?
 */

use core::fmt;
use itertools::Itertools;

// constants and type aliases
const BOARD_SIZE: usize = 3;
type Board = [[Field; BOARD_SIZE]; BOARD_SIZE];

/// a field of a board which can be Empty
/// or have some kind of symbol on it.
/// a non-Empty field can be considered a player.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Field {
    /// field is empty
    Empty,
    Cross,
    Circle
}
use Field::*;

impl Field {
    /// get a character for each variant
    fn symbol(&self) -> char {
        match *self {
            Empty => '_',
            Cross => 'X',
            Circle => 'O'
        }
    }
}

// format field as specific symbol on display
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// current status of the game
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status<'a> {
    Draw,
    SomeoneWon(&'a Field), 
    StillPlaying
}
use Status::*;

// format status as message on display
impl<'a> fmt::Display for Status<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Draw => String::from("its a draw :("),
            StillPlaying => String::from("the game is still going..."),
            SomeoneWon(winner) => format!("{:?} won!", winner)
        })
    }
}

/// print the board using the field display format and spacers
fn print_board(board: &Board) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            print!("{}  ", board[i][j]);
        }
        println!();
    }
}

/// get the current status of a board
fn get_board_status(board: &Board) -> Status {
    /// return the field that won a line (row/column/diagonal, if present)
    fn someone_won<'a>(line: &[&'a Field]) -> Option<&'a Field> {
        // check if the whole line consists of the same field
        let only_field_in_line = line.iter().all_equal_value();
        // if so, return it as the winner
        if only_field_in_line.is_ok() {
            return Some(only_field_in_line.ok().expect(""));
        }
        // else return None
        None
    }

    /// check if a line can not be won by anyone anymore (draw)
    fn is_draw(line: &[&Field]) -> bool {
        // a line is a draw if there are at least two unique fields on it (excluding Empty)
        // TODO implement function
        false
    }

    /// returns rows of board
    fn get_rows(board: &Board) -> Vec<[&Field; BOARD_SIZE]> {
        // make an effort to get the right return type
        board.iter()
            .map(|row| {
                let mut array = [&Empty; BOARD_SIZE];
                for i in 0 .. BOARD_SIZE {
                    array[i] = &row[i];
                }
                array
            }).collect_vec()
    }

    /// returns columns of board
    fn get_columns(board: &Board) -> Vec<[&Field; BOARD_SIZE]> {
        let mut columns = Vec::new();
        // for all columns
        for col_i in 0 .. BOARD_SIZE {
            // get column from board
            let mut column = [&Empty; BOARD_SIZE];
            for row_i in 0 .. BOARD_SIZE {
                column[row_i] = &board[row_i][col_i];
            }
            columns.push(column);
        }
        columns
    }

    /// returns diagonals of board
    fn get_diagonals(board: &Board) -> Vec<[&Field; BOARD_SIZE]> {
        let mut diagonals = Vec::new();
        // for all diagonals
        // diagonal_factor will be used to get the two possible diagonals
        for diagonal_factor in [0, BOARD_SIZE - 1] {
            // get diagonal from board
            let mut diagonal: [&Field; BOARD_SIZE] = [&Empty; BOARD_SIZE];
            // for all rows/columns
            for i in 0 .. BOARD_SIZE {
                // use diagonal factor to get major and minor diagonal values
                diagonal[i] = &board[i][i.abs_diff(diagonal_factor)];
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
    .collect_vec();

    // return the first winner if there are winners
    if winners.len() != 0 {
        return SomeoneWon(winners[0].expect(""));
    }

    Status::StillPlaying
}

fn main() {
    // new empty board
    let mut board: Board = [[Empty; BOARD_SIZE]; BOARD_SIZE];

    // make some moves
    board[0][0] = Circle;
    board[1][1] = Circle;
    board[2][2] = Circle;

    print_board(&board);
    println!("{}", get_board_status(&board));
}