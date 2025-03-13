use rtapt::char::assert_char_is_alphabetic;
use std::panic;

fn is_alphabetic(a_char: char) -> &'static str {
    let result = std::panic::catch_unwind(|| {
        assert_char_is_alphabetic(a_char);
    });

    if result.is_ok() {
        return &format!("The character '{}' is present in the alphabet.", a_char)
    }

    return &format!("The character '{}' does not exist in the alphabet.", a_char)
}

fn main() {
    let a_char = 'A';
    println!("Checking if '{}' is an alphabetic character...", a_char);
    println!("{}", is_alphabetic(a_char)); // The character 'A' is present in the alphabet.

    a_char = '1';
    println!("Checking if '{}' is an alphabetic character...", a_char);
    println!("{}", is_alphabetic(a_char)); // The character '1' does not exist in the alphabet.
}
