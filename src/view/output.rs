use crate::model::*;
use crate::constants::*;
use itertools::Itertools;

/// print content in a board-like format. panics if content
/// does not have exactly `FIELD_COUNT` elements.
fn print_with_board_format(content: Vec<String>, arrow_underneath: bool) {
    if content.len() != FIELD_COUNT.into() {
        panic!("content needs to have exactly {} elements", FIELD_COUNT);
    }

    // space between prints
    const SPACER: &str = " ";
    // how many spacers should be used between fields of a Board
    const SPACE_REPS: usize = 2;
    // max length of field index
    let field_length = FIELD_COUNT.to_string().len();

    let mut index = 0;
    for _ in BOARD_RANGE {
        print!("|{}", SPACER);
        for col in BOARD_RANGE {
            print!("{:>length$}", content[index], length = field_length);
            if col != BOARD_USIZE - 1 { print!("{}", SPACER.repeat(SPACE_REPS)); }
            index += 1;
        }
        println!("{}|", SPACER);
    }

    if !arrow_underneath { return; }

    let line_length =
        field_length * BOARD_USIZE + // content
        SPACER.len() * 2 + // left and right space
        SPACER.len() * (BOARD_USIZE - 1) * SPACE_REPS +
        2; // pipe chars
    println!("{:-^length$}", "v-v-v", length = line_length);
}

pub fn clear_terminal() {
    clearscreen::clear().expect("failed to clear terminal");
}

/// print the board using the field display format and spacers
pub fn print_board(board: &Board) {
    print_with_board_format(
        board.iter()
            .flatten()
            .map(|field|
                field.unwrap_or(' ').to_string())
            .collect_vec(),
        false
    );
}

/// print board with indices of fields and an arrow pointing downwards underneath
pub fn print_board_template() {
    print_with_board_format(
        (1 ..= FIELD_COUNT)
            .map(|index| index.to_string())
            .collect_vec(),
        true
    );
}