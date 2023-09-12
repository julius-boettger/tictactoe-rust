/* TODO's
 * - select number of players and their symbols at startup
 * - cycle between players to make moves
 * - add error handling for too big BOARD_SIZE
 * - introduce MAX_PLAYERS
 */

mod view;
mod control;
mod model;
mod constants;

fn main() {
    control::run_game();
}