use rtapt::char::assert_char_is_alphanumeric;
use rtapt::char::constants::ALPHA_NUMERIC;
use std::panic;

#[cfg(test)]
mod char_tests {
    use super::*;

    #[test]
    fn assert_char_is_alphanumeric_not_panic() {
        for &first_char in &ALPHA_NUMERIC {
            assert_char_is_alphanumeric(first_char);
        }
    }

    #[test]
    fn assert_char_is_alphanumeric_panic() {
        let result = panic::catch_unwind(|| {
            assert_char_is_alphanumeric('!');
        });

        if result.is_ok() {
            panic!("assert_char_is_alphanumeric: Expected panic, but it did not occur!");
        }
    }
}
