use rtapt::char::assert_eq_char;
use rtapt::char::constants::ALL_CHARACTERS;
use std::panic;

#[cfg(test)]
mod char_tests {
    use super::*;

    #[test]
    fn assert_eq_char_not_panic() {
        let mut first_char: char;
        let mut second_char: char;

        for iterator in 0..ALL_CHARACTERS.len() {
            first_char = ALL_CHARACTERS[iterator];
            second_char = ALL_CHARACTERS[iterator];

            assert_eq_char(first_char, second_char);
        }
    }

    #[test]
    fn assert_eq_char_panic() {
        let result = panic::catch_unwind(|| {
            assert_eq_char('A', 'đ');
        });

        if result.is_ok() {
            panic!("assert_eq_char_panic: Expected panic, but it did not occur!");
        }
    }
}
