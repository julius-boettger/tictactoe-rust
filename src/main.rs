/* TODO
 * - extract strings to own module
 * - clear terminal after every error message
 * - make BOARD_SIZE changeable at runtime, e.g. by wrapping game in a struct
 * 
 * RELEASE COMPILE
 * cross build --release --target x86_64-unknown-linux-gnu --target aarch64-unknown-linux-gnu --target riscv64gc-unknown-linux-gnu --target x86_64-pc-windows-gnu
 */

mod view;
mod control;
mod model;
mod constants;

fn main() {
    control::run_game();
}