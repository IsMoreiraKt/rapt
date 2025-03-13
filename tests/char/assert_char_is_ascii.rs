use rtapt::char::assert_char_is_ascii;
use rtapt::char::constants::ASCII;
use std::panic;

#[cfg(test)]
mod char_tests {
    use super::*;

    #[test]
    fn assert_char_is_ascii_not_panic() {
        for &first_char in &ASCII {
            assert_char_is_ascii(first_char);
        }
    }

    #[test]
    fn assert_char_is_ascii_panic() {
        let result = panic::catch_unwind(|| {
            assert_char_is_ascii('λ');
        });

        if result.is_ok() {
            panic!("assert_char_is_ascii: Expected panic, but it did not occur!");
        }
    }
}
