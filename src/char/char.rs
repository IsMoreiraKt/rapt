//! @file char.rs
//! This file is part of the RTAPT library.
//!
//! @brief
//! Contains assertions specific for the `char` type in the RTAPT library.
//!
//! @license RTAPT is licensed under the LGPL-2.1-or-later.
//!
//! @author Ismael Gomes Moreira <ismaelmoreirakt@gmail.com>
use crate::char::constants::*;

/// Verifies if two `char`s are equal. Panics if they are not.
///
/// # Arguments
/// * `first_char` - The first character to compare.
/// * `second_char` - The second character to compare.
///
/// # Panics
/// Panics if the characters are different.
pub fn assert_char_eq(first_char: char, second_char: char) {
    if first_char != second_char {
        panic!(
            "assert_char_eq: '{}' is different from '{}'",
            first_char, second_char
        );
    }
}

/// Verifies if a character is alphabetic. Panics if it is not.
///
/// # Arguments
/// * `a_char` - The character to check.
///
/// # Panics
/// Panics if the character is not alphabetic.
pub fn assert_char_is_alphabetic(a_char: char) {
    if !ALPHA.contains(&a_char) {
        panic!(
            "assert_char_is_alphabetic: '{}' is not an alphabetic character",
            a_char
        );
    }
}

/// Verifies if a character is alphanumeric. Panics if it is not.
///
/// This function checks if a given character is alphanumeric, meaning it is either
/// a letter or a digit. If the character is neither a letter nor a digit, it panics.
///
/// # Arguments
/// * `a_char` - The character to check.
///
/// # Panics
/// Panics if the character is not alphanumeric.
pub fn assert_char_is_alphanumeric(a_char: char) {
    if !ALPHA_NUMERIC.contains(&a_char) {
        panic!(
            "assert_char_is_alphanumeric: '{}' is not an alpha numeric character",
            a_char
        );
    }
}
