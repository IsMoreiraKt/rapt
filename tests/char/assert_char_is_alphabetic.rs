use rtapt::char::assert_char_is_alphabetic;
use rtapt::char::constants::ALPHA;
use std::panic;

#[cfg(test)]
mod char_tests {
    use super::*;

    #[test]
    fn assert_char_is_alphabetic_not_panic() {
        for &first_char in &ALPHA {
            assert_char_is_alphabetic(first_char);
        }
    }

    #[test]
    fn assert_char_is_alphabetic_panic() {
        let result = panic::catch_unwind(|| {
            assert_char_is_alphabetic('0');
        });

        if result.is_ok() {
            panic!("assert_char_is_alphabetic_panic: Expected panic, but it did not occur!");
        }
    }
}
