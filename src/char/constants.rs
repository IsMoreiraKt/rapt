//! @file constants.rs
//! This file is part of the RTAPT library.
//!
//! @brief
//! Contains constants for various character sets, including digits, letters,
//! symbols, and different character categories such as ASCII, Unicode,
//! and whitespace characters.
//!
//! @license RTAPT is licensed under the LGPL-2.1-or-later.
//!
//! @author Ismael Gomes Moreira <ismaelmoreirakt@gmail.com>

/// A constant array containing all digits from '0' to '9'.
///
/// This array is useful for operations where you need to reference or
/// check individual digit characters in a string or collection.
pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// A constant array containing all lowercase letters from 'a' to 'z'.
///
/// This array can be used to validate, filter, or operate on
/// lowercase alphabetic characters.
pub const LOWERCASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// A constant array containing all uppercase letters from 'A' to 'Z'.
///
/// Use this array when you need to check or manipulate uppercase
/// alphabetic characters.
pub const UPPERCASE_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// A constant array containing a set of non-ASCII characters representing
/// other letters.
///
/// This array includes letters like 'ņ', 'ķ', and others commonly used
/// in languages that extend beyond the basic ASCII character set.
pub const OTHER_LETTERS: [char; 116] = [
    'ņ', 'ȩ', 'ŗ', 'ţ', 'ş', 'ḑ', 'ģ', 'ḩ', 'ķ', 'ļ', 'ç', 'ẅ', 'ë', 'ẗ', 'ÿ', 'ü', 'ï', 'ö', 'ä',
    'ḧ', 'ẍ', 'ã', 'ð', 'đ', 'ŋ', 'ħ', 'ĸ', 'ł', 'ẽ', 'ỹ', 'ũ', 'ĩ', 'õ', 'ṽ', 'ñ', 'Ẽ', 'Ỹ', 'Ũ',
    'Ĩ', 'Õ', 'Ṽ', 'Ñ', 'Ã', 'ŵ', 'ê', 'ŷ', 'û', 'î', 'ô', 'â', 'ŝ', 'ĝ', 'ĥ', 'ẑ', 'ĉ', 'Ŵ', 'Ê',
    'Ŷ', 'Û', 'Î', 'Ô', 'Â', 'Ŝ', 'Ĝ', 'Ĥ', 'Ẑ', 'Ĉ', 'ẉ', 'ẹ', 'ṛ', 'ṭ', 'ỵ', 'ụ', 'ị', 'ọ', 'ạ',
    'ṣ', 'ḍ', 'ḥ', 'ḳ', 'ḷ', 'ẓ', 'ṿ', 'ḅ', 'ṇ', 'ṃ', 'Ẉ', 'Ẹ', 'Ṛ', 'Ṭ', 'Ỵ', 'Ụ', 'Ị', 'Ọ', 'Ạ',
    'Ṣ', 'Ḍ', 'Ḥ', 'Ḳ', 'Ḷ', 'Ẓ', 'Ṿ', 'Ḅ', 'Ṇ', 'Ṃ', 'Ŧ', 'Ø', 'Þ', 'Æ', 'ẞ', 'Ð', 'Ŋ', 'Ħ', 'Ł',
    'æ', 'ß',
];

/// A constant array containing common whitespace characters: space, tab,
/// and newline.
///
/// This array is used when filtering or identifying whitespace characters
/// in strings or data streams.
pub const WHITESPACE: [char; 3] = [' ', '\t', '\n'];

/// A constant array containing all alphabetic characters
/// (both uppercase and lowercase).
///
/// This is useful for operations where you need to validate or filter
/// alphabetic characters, regardless of case.
pub const ALPHA: [char; LOWERCASE_LETTERS.len() + UPPERCASE_LETTERS.len()] = {
    let mut alpha = [0 as char; 52];
    let mut iterator = 0;

    // Copy lowercase letters to the first half
    while iterator < 26 {
        alpha[iterator] = LOWERCASE_LETTERS[iterator];
        iterator += 1;
    }

    iterator = 0;

    // Copy uppercase letters to the second half
    while iterator < 26 {
        alpha[26 + iterator] = UPPERCASE_LETTERS[iterator];
        iterator += 1;
    }

    alpha
};

/// A constant array containing all alphanumeric characters
/// (digits, lowercase, and uppercase letters).
///
/// This is commonly used for validating usernames, identifiers, or any
/// input requiring only alphanumeric characters.
pub const ALPHA_NUMERIC: [char; ALPHA.len() + DIGITS.len()] = {
    let mut alpha_numeric = [0 as char; 62];
    let mut iterator = 0;

    while iterator < 10 {
        alpha_numeric[iterator] = DIGITS[iterator];
        iterator += 1;
    }

    let mut index = 10;
    let mut j_iterator = 0;

    while iterator < 62 {
        alpha_numeric[index] = ALPHA[j_iterator];
        iterator += 1;
        j_iterator += 1;
        index += 1;
    }

    alpha_numeric
};

/// A constant array containing a variety of common symbols and special
/// characters used in programming and text.
///
/// This is useful when filtering or validating characters in programming
/// languages, markup, or other text-based systems.
pub const SYMBOLS: [char; 62] = [
    '\\', '|', '¬', '!', '¹', '"', '@', '#', '£', '$', '§', '%', '½', '&', '/', '{', '(', '[', ')',
    ']', '=', '}', '\'', '?', '«', '»', '¸', 'ſ', '€', '¶', 'ŧ', '←', '↓', 'ø', 'þ', '*', '+', '¨',
    '`', '´', 'ˀ', 'º', 'ª', '~', '^', '<', '>', '«', '»', '¢', '„', '“', '”', 'µ', ',', ';', '•',
    '.', ':', '·', '-', '_',
];

/// A constant array containing all ASCII characters (values from 0 to 127).
///
/// This array represents the basic ASCII character set, including
/// control characters, printable characters, and non-printable characters.
pub const ASCII: [char; 128] = {
    let mut ascii = [0 as char; 128];
    let mut iterator = 0;

    while iterator < 128 {
        ascii[iterator] = iterator as u8 as char;
        iterator += 1;
    }

    ascii
};

/// A constant array containing all printable ASCII characters
/// (values from 32 to 126).
///
/// This is used when you want to work with just the visible characters
/// in the ASCII range.
pub const PRINTABLE_ASCII: [char; 95] = {
    let mut printable = [0 as char; 95];
    let mut iterator = 32;
    let mut index = 0;

    while iterator <= 126 {
        printable[index] = iterator as u8 as char;
        iterator += 1;
        index += 1;
    }

    printable
};

/// A constant array containing all non-printable ASCII characters
/// (values from 0 to 31).
///
/// This is useful when dealing with control characters or data streams
/// that may include non-visible characters.
pub const NON_PRINTABLE_ASCII: [char; 32] = {
    let mut non_printable = [0 as char; 32];
    let mut iterator = 0;

    while iterator < 32 {
        non_printable[iterator] = iterator as u8 as char;
        iterator += 1;
    }

    non_printable
};

/// A constant array containing printable Unicode characters
/// (values from U+0020 to U+007F).
///
/// This is similar to `PRINTABLE_ASCII`, but explicitly includes the
/// Unicode characters in the range U+0020 to U+007F.
pub const UNICODE_PRINTABLE: [char; 128] = {
    let mut unicode_printable = [0 as char; 128];
    let mut iterator = 0x20;
    let mut index = 0;

    while iterator <= 0x7F {
        unicode_printable[index] = char::from_u32(iterator).unwrap();
        iterator += 1;
        index += 1;
    }

    unicode_printable
};

/// A constant array containing all characters from various categories
/// (digits, letters, symbols, etc.).
///
/// This array combines all the defined character sets into one large
/// array for operations that need to work with any type of character.
pub const ALL_CHARACTERS: [char;
    OTHER_LETTERS.len()
        + WHITESPACE.len()
        + ALPHA_NUMERIC.len()
        + SYMBOLS.len()
        + ASCII.len()
        + UNICODE_PRINTABLE.len()] = {
    let mut all_chars = [0 as char;
        OTHER_LETTERS.len()
            + WHITESPACE.len()
            + ALPHA_NUMERIC.len()
            + SYMBOLS.len()
            + ASCII.len()
            + UNICODE_PRINTABLE.len()];
    let mut iterator = 0;
    let mut j_iterator = 0;

    while j_iterator < OTHER_LETTERS.len() {
        all_chars[iterator] = OTHER_LETTERS[j_iterator];
        iterator += 1;
        j_iterator += 1;
    }

    j_iterator = 0;
    while j_iterator < WHITESPACE.len() {
        all_chars[iterator] = WHITESPACE[j_iterator];
        iterator += 1;
        j_iterator += 1;
    }

    j_iterator = 0;
    while j_iterator < ALPHA_NUMERIC.len() {
        all_chars[iterator] = ALPHA_NUMERIC[j_iterator];
        iterator += 1;
        j_iterator += 1;
    }

    j_iterator = 0;
    while j_iterator < SYMBOLS.len() {
        all_chars[iterator] = SYMBOLS[j_iterator];
        iterator += 1;
        j_iterator += 1;
    }

    j_iterator = 0;
    while j_iterator < ASCII.len() {
        all_chars[iterator] = ASCII[j_iterator];
        iterator += 1;
        j_iterator += 1;
    }

    j_iterator = 0;
    while j_iterator < UNICODE_PRINTABLE.len() {
        all_chars[iterator] = UNICODE_PRINTABLE[j_iterator];
        iterator += 1;
        j_iterator += 1;
    }

    all_chars
};
