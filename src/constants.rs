/// number of rows and columns of the tictactoe board
pub const BOARD_SIZE: u8 = 4;
/// number of rows and columns of the tictactoe board (as `usize`)
pub const BOARD_USIZE: usize = BOARD_SIZE as usize;
/// for iterating through board rows or colums
pub const BOARD_RANGE: std::ops::Range<usize> = 0 .. BOARD_USIZE;
/// number of fields on the tictactoe board
pub const FIELD_COUNT: u8 = BOARD_SIZE.pow(2);