//! Utility functions for recognising tokens.

/// True if `unit` is within '0' and '9'.
pub fn is_decimal_digit(unit: char) -> bool {
    unit.is_ascii_digit()
}

/// True if `unit` is a whitespace character, including newlines.
pub fn is_whitespace(unit: char) -> bool {
    unit.is_whitespace()
}

/// True if `unit` is an ASCII alphabetical character.
pub fn is_alpha(unit: char) -> bool {
    unit.is_ascii_alphabetic()
}

// True if `unit` is an ASCII alphabetical character or an ASCII decimal digit.
pub fn is_alphanum(unit: char) -> bool {
    unit.is_ascii_alphanumeric()
}

/// True if `unit` is '.'
pub fn is_fullstop(unit: char) -> bool {
    unit == '.'
}

/// True if `unit` is a character used in one of the operator symbols.
/// 
/// For now this is restricted to the following characters:
/// 1. +
/// 2. -
/// 3. *
/// 4. /
pub fn is_opchar(unit: char) -> bool {
    matches!(unit, '+' | '-' | '*' | '/')
}
