use crate::model::*;
use crate::constants::*;
use itertools::Itertools;

/// print the board using the field display format and spacers
pub fn print_board(board: &Board) {
    print_with_board_format(
        board.iter()
        .flatten()
        .map(|f| f.unwrap_or(' ').to_string())
        .collect_vec(), false
    );
}

/// print board with indices of fields and an arrow pointing downwards underneath
pub fn print_board_template() {
    print_with_board_format(
        (1 ..= BOARD_SIZE.pow(2))
        .map(|i| i.to_string())
        .collect_vec(), true
    );
}

/// print content in a board-like format. panics if content
/// does not have exactly `BOARD_SIZE.pow(2)` elements.
fn print_with_board_format(content: Vec<String>, arrow_underneath: bool) {
    if content.len() != BOARD_SIZE.pow(2) {
        panic!("content needs to have exactly {} elements", BOARD_SIZE.pow(2));
    }

    // space between prints
    const SPACER: &str = " ";
    // how many spacers should be used between fields of a Board
    const SPACE_REPS: usize = 2;
    // max length of field index
    let field_length = BOARD_SIZE.pow(2).to_string().len();

    let mut index = 0;
    for _ in 0..BOARD_SIZE {
        print!("|{}", SPACER);
        for col in 0..BOARD_SIZE {
            print!("{:>length$}", content[index], length = field_length);
            if col != BOARD_SIZE - 1 { print!("{}", SPACER.repeat(SPACE_REPS)); }
            index += 1;
        }
        println!("{}|", SPACER);
    }

    if !arrow_underneath { return; }

    let line_length =
        field_length * BOARD_SIZE + // content
        SPACER.len() * 2 + // left and right space
        SPACER.len() * (BOARD_SIZE - 1) * SPACE_REPS +
        2; // pipe chars
    println!("{:-^length$}", "v-v-v", length = line_length);
}