use crate::model::*;
use Status as S;

use crate::view;
use crate::constants::*;
use itertools::Itertools;

/// return the field that won a line (row/column/diagonal, if present)
fn get_line_winner(line: &[Field]) -> Field {
    let only_field_in_line: Result<&Field, _> = line.iter().all_equal_value();
    if let Ok(Some(winner)) = only_field_in_line {
        return Some(*winner);
    }
    None
}

/// check if a line can not be won by anyone anymore (draw)
fn get_line_draw(line: &[Field]) -> bool {
    // a line is a draw if there are at least two unique fields on it (excluding Empty)
    line.iter()
        .unique()
        .filter(|field| field.is_some())
        .collect_vec()
        .len() >= 2
}

/// returns rows of board
fn get_rows(board: &Board) -> Vec<Line> {
    board.iter()
         .map(|&row| row)
         .collect_vec()
}

/// returns columns of board
fn get_columns(board: &Board) -> Vec<Line> {
    let mut columns: Vec<Line> = Vec::new();
    for col_i in BOARD_RANGE {
        let mut column: Line = [None; BOARD_USIZE];
        for row_i in BOARD_RANGE {
            column[row_i] = board[row_i][col_i];
        }
        columns.push(column);
    }
    columns
}

/// returns diagonals of board
fn get_diagonals(board: &Board) -> Vec<Line> {
    let mut diagonals: Vec<Line> = Vec::new();
    // diagonal_factor will be used to get the two possible diagonals
    for diagonal_factor in [0, BOARD_USIZE - 1] {
        // get diagonal from board
        let mut diagonal: Line = [None; BOARD_USIZE];
        // for all rows/columns
        for line in BOARD_RANGE {
            // use diagonal factor to get major and minor diagonal values
            diagonal[line] = board[line][line.abs_diff(diagonal_factor)];
        }
        diagonals.push(diagonal)
    }
    diagonals
}

/// get all lines of the board (rows, columns and diagonals)
fn get_board_lines(board: &Board) -> Vec<Line> {
    let mut lines = Vec::new();
    lines.append(&mut get_rows(board));
    lines.append(&mut get_columns(board));
    lines.append(&mut get_diagonals(board));
    lines
}

/// get first winner of board
fn get_board_winner(board_lines: &Vec<Line>) -> Field {
    let winners = board_lines.iter()
        .map(|line| get_line_winner(line))
        .filter(|field| field.is_some())
        .map(|option| option.unwrap())
        .collect_vec();

    if !winners.is_empty() {
        return Some(winners[0]);
    }

    None
}

/// check if board is a draw
fn get_board_draw(board_lines: &Vec<Line>) -> bool {
    // check the board for a draw (all lines have to be draws)
    let draw = board_lines.iter()
        .map(|line| get_line_draw(line))
        .all_equal_value();

    // if all lines have the same draw value and it is true
    if let Ok(value) = draw {
        if value {
            return true;
        }
    }

    false
}

/// get the current status of a board
fn get_board_status(board: &Board) -> Status {

    let board_lines = get_board_lines(board);

    if let Some(field) = get_board_winner(&board_lines) {
        return S::SomeoneWon(field);
    }

    if get_board_draw(&board_lines) {
        return S::Draw;
    }

    // the game is still going if neither a winner nor a draw could be determined
    Status::StillPlaying
}

/// construct a board. if content is `Some` it must be a vector with length `FIELD_COUNT`.
fn construct_board(content: Option<Vec<Field>>) -> Board {
    let mut board: Board = [[None; BOARD_USIZE]; BOARD_USIZE];

    let content = match content {
        Some(value) => value,
        None => return board
    };

    if content.len() == FIELD_COUNT.into() {
        let mut index = 0;
        for row in BOARD_RANGE {
            for col in BOARD_RANGE {
                board[row][col] = content[index];
                index += 1;
            }
        }
    }

    board
}

/// place a symbol on the board. `index` must be `>= 1` and `<= FIELD_COUNT`.
/// return `false` if field of `index` is not empty and the symbol wasn't placed.
fn place_symbol(board: &mut Board, index: u8, symbol: char) -> bool {
    if index < 1 || index > FIELD_COUNT {
        panic!("index is {}, but must be between 1 and FIELD_COUNT (=> {})",
            index, FIELD_COUNT);
    }

    let index = index - 1;
    let row: usize = (index / BOARD_SIZE).into();
    let col: usize = (index % BOARD_SIZE).into();

    if board[row][col].is_some() { return false; }

    board[row][col] = Some(symbol);
    true
}

/// prompt player to choose number of players with a symbol each.
/// return vector players presented by a symbol each.
fn get_players() -> Vec<char> {
    use view::*;
    let num_players = input::get_input_u8("number of players: ", 2, MAX_PLAYERS);
    let mut players: Vec<char> = Vec::new();
    for player in 1 ..= num_players {
        let info = format!("symbol for player {}: ", player);
        let symbol = input::get_input_char(info.as_str(), &players);
        players.push(symbol);
    }
    players
}

/// run a game of tic tac toe
pub fn run_game() {

    let max_board_size: u8 = f32::sqrt(u8::MAX as f32) as u8;
    if BOARD_SIZE < 3 || BOARD_SIZE > max_board_size {
        panic!("BOARD_SIZE is {}, but must be between 3 and {}", BOARD_SIZE, max_board_size);
    }

    let mut board: Board = construct_board(None);

    use view::*;

    let players = get_players();

    // game loop
    'outer: loop {
        for player_symbol in players.iter() {
            output::clear_terminal();

            output::print_board_template();
            output::print_board(&board);

            let board_status = get_board_status(&board);
            if board_status != S::StillPlaying {
                println!("{}", board_status);
                break 'outer;
            }

            let info = format!("player {}, make your move: ", player_symbol);
            let index = input::get_input_u8(info.as_str(), 1, FIELD_COUNT);
            let mut symbol_placed = place_symbol(&mut board, index, *player_symbol);
            // repeat until index points to an empty field
            while !symbol_placed {
                println!("that field is not empty!");
                let index = input::get_input_u8(info.as_str(), 1, FIELD_COUNT);
                symbol_placed = place_symbol(&mut board, index, *player_symbol);
            }
        }
    }

    input::get_input("the game is over. press enter to close this window.");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_board() {
        let board = construct_board(None);
        let is_empty = board.iter()
            .flatten()
            .all(|field| field.is_none());
        if !is_empty { panic!("board is not empty"); }
        assert_eq!(get_board_status(&board), S::StillPlaying);
    }

    #[test]
    fn place_symbol_test() {
        let mut board = construct_board(None);
        place_symbol(&mut board, 2, 'X');
        assert_eq!(board[0][1], Some('X'));
        assert_eq!(get_board_status(&board), S::StillPlaying);
    }

    #[test]
    fn multiple_winners() {
        let mut board = construct_board(None);
        board[0] = [Some('X'); BOARD_USIZE];
        board[1] = [Some('X'); BOARD_USIZE];
        assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
    }

    #[test]
    fn winner_in_rows() {
        for winner_row in BOARD_RANGE {
            let mut board = construct_board(None);
            board[winner_row] = [Some('X'); BOARD_USIZE];
            assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
        }
    }

    #[test]
    fn winner_in_cols() {
        for winner_col in BOARD_RANGE {
            let mut board = construct_board(None);
            for row in BOARD_RANGE {
                board[row][winner_col] = Some('X');
            }
            assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
        }
    }

    #[test]
    fn winner_in_diagonals() {
        for diagonal_factor in [0, BOARD_USIZE - 1] {
            let mut board = construct_board(None);
            for line in BOARD_RANGE {
                board[line][line.abs_diff(diagonal_factor)] = Some('X');
            }
            assert_eq!(get_board_status(&board), S::SomeoneWon('X'));
        }
    }
}