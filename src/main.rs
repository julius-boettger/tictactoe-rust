/* TODO
 * - extract strings to own module
 * - clear terminal after every error message
 */

mod view;
mod control;
mod model;
mod constants;

fn main() {
    control::run_game();
}