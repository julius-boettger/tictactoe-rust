use crate::constants::*;

/// a field of a board which can be empty (None)
/// or have some kind of symbol on it.
/// a non-empty field can be considered a player.
pub type Field = Option<char>;
/// a line of a tictactoe board consisting of multiple Field's
pub type Line = [Field; BOARD_USIZE];
/// a tictactoe board consisting of multiple Line's
pub type Board = [Line; BOARD_USIZE];
/// a move to make on the board, consisting of the field
/// index (`>= 1 && <= FIELD_COUNT`) and a player symbol
pub type Move = (u8, char);

/// current status of the game
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Draw,
    StillPlaying,
    SomeoneWon(char)
}