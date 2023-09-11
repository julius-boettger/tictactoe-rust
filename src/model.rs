use crate::constants::*;

pub type Line = [crate::model::Field; BOARD_SIZE];
pub type Board = [Line; BOARD_SIZE];

/// a field of a board which can be empty (None)
/// or have some kind of symbol on it.
/// a non-empty field can be considered a player.
pub type Field = Option<char>;

/// current status of the game
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Draw,
    SomeoneWon(char), 
    StillPlaying
}