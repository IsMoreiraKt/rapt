//! @file char.rs
//! This file is part of the RTAPT library.
//!
//! @brief
//! Contains assertions specific for the `char` type in the RTAPT library.
//!
//! @license RTAPT is licensed under the LGPL-2.1-or-later.
//!
//! @author Ismael Gomes Moreira <ismaelmoreirakt@gmail.com>

/// Verifies if two `char`s are equal. If they are not, it panics.
///
/// # Arguments
///
/// * `first_char` - The first character to compare.
/// * `second_char` - The second character to compare.
pub fn assert_eq_char(first_char: char, second_char: char) {
    if first_char != second_char {
        panic!(
            "assert_eq_char: '{}' is different from '{}'",
            first_char, second_char
        );
    }
}
