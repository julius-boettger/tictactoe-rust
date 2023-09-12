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

/// get a `u8` through stdin after printing `info`.
/// repeat recursively until input is between `min` and `max` (both included).
pub fn get_input_u8(info: &str, min: u8, max: u8) -> u8 {
    let error = format!(
        "your input could not be recognized as a number between {} and {}",
        min, max);
    let predicate = |num: &u8| *num >= min && *num <= max;
    get_input_type(info, error.as_str(), error.as_str(), predicate)
}

/// get a `char` through stdin after printing `info`.
/// repeat recursively until input is not empty, some kind of whitespace or contained in `forbidden`.
pub fn get_input_char(info: &str, forbidden: &Vec<char>) -> char {
    let mut error =
        "your input could not be recognized as a single character like 'x'"
        .to_string();
    if !forbidden.is_empty() {
        error += format!(" that is not one of {:?}", forbidden).as_str();
    }
    let predicate = |symbol: &char| !forbidden.contains(symbol);
    get_input_type(info, error.as_str(), error.as_str(), predicate)
}