use rtapt::char::assert_char_is_alphanumeric;

fn is_alphanumeric(a_char: char) -> String {
    let result = std::panic::catch_unwind(|| {
        assert_char_is_alphanumeric(a_char);
    });

    if result.is_ok() {
        return format!("The '{}' character is alphanumeric.", a_char);
    }

    format!("The '{}' character is not alphanumeric.", a_char)
}

fn main() {
    let a_char = 'A';
    println!("Checking if the character '{}' is alphanumeric...", a_char);
    println!("{}", is_alphanumeric(a_char)); // The 'A' character is alphanumeric.

    a_char = '0';
    println!("Checking if the character '{}' is alphanumeric...", a_char);
    println!("{}", is_alphanumeric(a_char)); // The '0' character is alphanumeric.

    let a_char = '!';
    println!("Checking if the character '{}' is alphanumeric...", a_char);
    println!("{}", is_alphanumeric(a_char)); // The '!' character is not alphanumeric.
}
