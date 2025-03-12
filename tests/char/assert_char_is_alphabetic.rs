use rtapt::char::assert_char_is_alphabetic;
use rtapt::char::constants::ALPHA;
use std::panic;

#[cfg(test)]
mod char_tests {
    use super::*;

    #[test]
    fn assert_char_is_alphabetic_not_panic() {
        let mut a_char: char;

        for iterator in 0..ALPHA.len() {
            a_char = ALPHA[iterator];

            assert_char_is_alphabetic(a_char);
        }
    }

    #[test]
    fn assert_char_is_alphabetic_panic() {
        let result = panic::catch_unwind(|| {
            assert_is_alphabetic('0');
        });

        if result.is_ok() {
            panic!("assert_is_alphabetic_panic: Expected panic, but it did not occur!");
        }
    }
}
