/// a field of a board which can be Empty
/// or have some kind of symbol on it.
/// a non-Empty field can be considered a player.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Field {
    /// field is empty
    Empty,
    X,
    O,
    Z,
    G
}

/// current status of the game
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Draw,
    SomeoneWon(Field), 
    StillPlaying
}