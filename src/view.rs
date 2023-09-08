use crate::model::Field;
use Field as F;
use crate::model::Status;
use Status as S;

use crate::constants::*;
use std::fmt;

// convert each field to a character (and reverse)
impl From<char> for Field {
    fn from(symbol: char) -> Self {
        match symbol {
            'X' => F::X,
            'O' => F::O,
            'Z' => F::Z,
            'G' => F::G,
             _  => F::Empty
        }
    }
}
impl From<Field> for char {
    fn from(field: Field) -> Self {
        match field {
            F::X     => 'X',
            F::O     => 'O',
            F::Z     => 'Z',
            F::G     => 'G',
            F::Empty => ' '
        }
    }
}

// format field as specific symbol on display
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

// format status as message on display
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            S::Draw => String::from("its a draw! no player can win anymore."),
            S::StillPlaying => String::from("the game is still going..."),
            S::SomeoneWon(winner) => format!("{} won!", winner)
        })
    }
}

/// print the board using the field display format and spacers
pub fn print_board(board: &Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            print!("{}  ", board[row][col]);
        }
        println!();
    }
}

/// get a line of user input through stdin after printing an info-text
pub fn get_input(info: &str) -> String {
    use std::io::{stdin, stdout, Write};
    // print info text and flush the output
    print!("{info}");
    stdout().flush().expect("something went wrong when flushing stdout");
    // read input
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("something went wrong reading stdin");
    // return input without newline at end
    buffer.trim_end().into()
}