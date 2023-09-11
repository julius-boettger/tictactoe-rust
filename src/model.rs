use crate::constants::*;

/// a field of a board which can be empty (None)
/// or have some kind of symbol on it.
/// a non-empty field can be considered a player.
pub type Field = Option<char>;
/// a line of a tictactoe board consisting of multiple Field's
pub type Line = [Field; BOARD_SIZE];
/// a tictactoe board consisting of multiple Line's
pub type Board = [Line; BOARD_SIZE];

/// current status of the game
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Draw,
    StillPlaying,
    SomeoneWon(char)
}