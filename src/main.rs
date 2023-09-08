/* TODO's
 * - give expect("") real messages or just remove it
 * - improve modularity
 * - make game playable through terminal
 * - ask user how many players should be used
 * - write tests
 */

mod view;
mod control;
mod model;
mod constants;

fn main() {
    control::run_game();
}
