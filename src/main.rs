/* TODO's
 * - give expect("") real messages
 * - row.iter.collect_vec() could be better?
 */

use core::fmt;
use itertools::Itertools;

// constants and type aliases
const BOARD_SIZE: usize = 3;
type Board = [[Field; BOARD_SIZE]; BOARD_SIZE];

/// a field of a board which can be empty
/// or have some kind of symbol on it
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
enum Status {
    Draw,
    SomeoneWon(Field),
    StillPlaying
}
use Status::*;

// format status as message on display
impl fmt::Display for Status {
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
    /// return the field that won a row/column/diagonal (if present)
    fn someone_won(fields: Vec<&Field>) -> Option<&Field> {
        let unique_fields = fields.iter()
            .unique()
            // dereference once
            .map(|field| *field)
            .collect_vec();
        // return if not just one unique element
        if unique_fields.len() != 1 { return None; }
        // get only field
        let field = unique_fields[0];
        // return if field is Empty
        if *field == Field::Empty { return None; }
        // return field that won
        Some(field)
    }

    /// returns the first field that won a row on the board (if present)
    fn winner_in_row(board: &Board) -> Option<&Field> {
        // vector containing all winners (of all rows)
        let winners = board.iter()
            // map each row to its potential winner
            .map(|row| someone_won(row.iter().collect_vec()))
            // filter out None's
            .filter(|option| option.is_some())
            .collect_vec();
        // return None if there were no winners
        if winners.len() == 0 { return None; }
        // return first winner
        else { return winners[0]; }
    }

    /// returns the first field that won a column on the board (if present)
    fn winner_in_column(board: &Board) -> Option<&Field> {
        // for all columns
        for col_i in 0 .. BOARD_SIZE {
            // get column from board
            let mut column: [&Field; BOARD_SIZE] = [&Empty; BOARD_SIZE];
            for row_i in 0 .. BOARD_SIZE {
                column[row_i] = &board[row_i][col_i];
            }
            // get potential winner
            let winner = someone_won(column.to_vec());
            // return winner if present
            if winner.is_some() {
                return winner;
            }
        }
        // return None if there were no winners
        return None;
    }

    // TODO combine possible results into array for figuring out winner?

    let winner_in_row = winner_in_row(board);
    if winner_in_row.is_some() {
        return Status::SomeoneWon(*winner_in_row.expect(""));
    }

    let winner_in_column = winner_in_column(board);
    if winner_in_column.is_some() {
        return Status::SomeoneWon(*winner_in_column.expect(""));
    }

    Status::StillPlaying
}

fn main() {
    // new empty board
    let mut board: Board = [[Empty; BOARD_SIZE]; BOARD_SIZE];

    // make some moves
    board[0][2] = Circle;
    board[1][2] = Circle;
    board[2][2] = Circle;

    print_board(&board);
    println!("{}", get_board_status(&board));
}