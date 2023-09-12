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

/// get a line of user input through stdin after printing `info`.
/// repeat recursively until successful.
/// try to parse the input to the given type `T`.
/// print `parse_error` if parsing fails.
/// call predicate with parsed value.
/// print `predicate_error` if predicate returns `false`.
pub fn get_input_type<T, F>(
    info: &str,
    parse_error: &str,
    predicate_error: &str,
    predicate: F
) -> T where
    T: std::str::FromStr,
    F: Fn(&T) -> bool
{
    let answer = get_input(info);
    let Ok(value) = answer.parse::<T>() else {
        println!("{}", parse_error);
        return get_input_type(info, parse_error, predicate_error, predicate);
    };
    if !predicate(&value) {
        println!("{}", predicate_error);
        return get_input_type(info, parse_error, predicate_error, predicate);
    }
    value
}