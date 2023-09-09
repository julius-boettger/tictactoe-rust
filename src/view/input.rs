/// get a line of user input through stdin after printing an info-text
pub fn get_input(info: &str) -> String {
    use std::io::{stdin, stdout, Write};
    // print info text and flush the output
    print!("{info}");
    stdout().flush().expect("something went wrong when flushing stdout");
    // read input
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("something went wrong reading stdin");
    // return input without newline at end
    buffer.trim_end().into()
}