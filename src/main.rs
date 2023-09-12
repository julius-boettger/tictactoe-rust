/* TODO
 * - extract strings to own module
 * - clear terminal after every error message
 * - make BOARD_SIZE changeable at runtime, e.g. by wrapping game in a struct
 */

mod view;
mod control;
mod model;
mod constants;

fn main() {
    control::run_game();
}