use rtapt::char::assert_char_is_ascii;
use std::panic;

fn is_ascii(a_char: char) -> String {
    let result = std::panic::catch_unwind(|| {
        assert_char_is_ascii(a_char);
    });

    if result.is_ok() {
        return format!("The '{}' character is ascii.", a_char);
    }

    format!("The '{}' character is not ascii.", a_char)
}

fn main() {
    let a_char = 'A';
    println!("Checking if the character '{}' is alphanumeric...", a_char);
    println!("{}", is_ascii(a_char)); // The 'A' character is ascii.

    a_char = 'λ';
    println!("Checking if the character '{}' is ascii...", a_char);
    println!("{}", is_ascii(a_char)); // The 'λ' character is ascii.
}
