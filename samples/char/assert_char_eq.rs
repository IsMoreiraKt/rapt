use rtapt::char::assert_char_eq;
use std::panic;

fn compare_chars(first_char: char, second_char: char) -> &'static str {
    let result = panic::catch_unwind(|| {
        assert_char_eq(first_char, second_char);
    });

    if result.is_ok() {
        "The two characters are the same."
    }

    "The characters are different."
}

fn main() {
    let first_char = '\t';
    let second_char = 'B';
    println!("{}", compare_chars(first_char, second_char)); // The characters are different.

    let first_char = 'A';
    let second_char = 'A';
    println!("{}", compare_chars(first_char, second_char)); // The two characters are the same.
}
