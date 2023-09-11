pub mod output;
pub mod input;

use crate::model::Status;
use Status as S;

use std::fmt;

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