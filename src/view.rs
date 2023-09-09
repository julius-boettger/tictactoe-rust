pub mod output;
pub mod input;

use crate::model::Field;
use Field as F;
use crate::model::Status;
use Status as S;

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