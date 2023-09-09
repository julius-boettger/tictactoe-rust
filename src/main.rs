/* TODO's
 * - make game playable through terminal
 * - ask user how many players should be used
 */

mod view;
mod control;
mod model;
mod constants;

fn main() {
    control::run_game();
}