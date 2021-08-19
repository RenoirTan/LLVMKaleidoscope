//! Utility functions for recognising tokens and checking whether they have
//! certain "characteristics" that makes them suitable to be part of a token
//! of a certain type.
//!
//! For example, an identifier must start with an ASCII letter (both miniscule
//! and majuscule) or an underscore `_`. Any following characters must be an
//! ASCII letter, underscore or numeric digit (0 to 9). If you want to ensure
//! that a given string can be a valid identifier, you can create a function
//! that checks the correctness of the identifier and return the index of
//! the first character that causes the string to fail the test. An example
//! implementation is shown below.
//!
//! ```
//! use kaleidoscope_lexer::utils;
//! 
//! fn validate_identifier(string: &str) -> (bool, Option<usize>) {
//!     if string.len() == 0 {
//!         (false, None)
//!     } else {
//!         let units: Vec<char> = string.chars().collect();
//!         if !utils::is_identifier_start(units[0]) {
//!             return (false, Some(0))
//!         }
//!         for (index, unit) in units[1..].iter().enumerate() {
//!             if !utils::is_identifier(*unit) {
//!                 return (false, Some(index+1))
//!             }
//!         }
//!         (true, None)
//!     }
//! }
//! 
//! let (ok, index) = validate_identifier("citizen_839293927392738");
//! assert!(ok); assert!(index.is_none());
//! let (ok, index) = validate_identifier("7huhfe");
//! assert!(!ok); assert!(matches!(index, Some(0)));
//! let (ok, index) = validate_identifier("");
//! assert!(!ok); assert!(index.is_none());
//! let (ok, index) = validate_identifier("hmm???");
//! assert!(!ok); assert!(matches!(index, Some(3)));
//! ```
//!
//! This is not the only application of the functions defined in this module.
//! You can tests for brackets, operators, numbers and punctuation, which play
//! an important role in the grammar of the language.

/// Rudimentary check to see if the end of the line has been reached.
pub fn is_eol(unit: char) -> bool {
    matches!(unit, '\n')
}

/// True if `unit` is within '0' and '9'.
pub fn is_decimal_digit(unit: char) -> bool {
    unit.is_ascii_digit()
}

/// True if `unit` is a whitespace character, including newlines.
pub fn is_whitespace(unit: char) -> bool {
    unit.is_whitespace()
}

/// True if `unit` is a character that can act as the first character of a name/identifier.
pub fn is_identifier_start(unit: char) -> bool {
    unit.is_ascii_alphabetic() || unit == '_'
}

/// True if `unit` is a character that can potentially be used in an identifier.
pub fn is_identifier(unit: char) -> bool {
    is_identifier_start(unit) || is_decimal_digit(unit)
}

/// True if `unit` is an ASCII alphabetical character.
pub fn is_alpha(unit: char) -> bool {
    unit.is_ascii_alphabetic()
}

/// True if `unit` is an ASCII alphabetical character or an ASCII decimal digit.
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
    matches!(unit, '+' | '-' | '*' | '/' | '<' | '>' | '=')
}

/// See if `unit` is the character that denotes the start of a comment.
pub fn is_comment(unit: char) -> bool {
    unit == '#'
}

/// See if `unit` is a bracket character.
pub fn is_bracket(unit: char) -> bool {
    matches!(unit, '(' | '[' | '{' | '<' | ')' | ']' | '}' | '>')
}

/// See if `unit` is a comma separator.
pub fn is_comma(unit: char) -> bool {
    unit == ','
}

/// See if `unit` is a fullstop.
pub fn is_dot(unit: char) -> bool {
    unit == '.'
}

/// See if `unit` is a semicolon.
pub fn is_semicolon(unit: char) -> bool {
    unit == ';'
}
