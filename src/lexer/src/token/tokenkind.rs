//! The type of token.
//! 
//! See [`TokenKind`].

use serde::{Serialize, Deserialize};
use super::{Keyword, Operator, Bracket};

/// The type of token a token is.
///
/// That's the worst explanation I have ever written in my life.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TokenKind {
    /// Default value for [`TokenKind`].
    /// Reserved for situations where the token kind cannot be guessed yet.
    Unknown,
    /// The token for when the file/stream is done.
    Eof,
    /// The token kind representing a keyword.
    Keyword { keyword: Keyword },
    /// The token kind representing an identifier.
    Identifier,
    /// The token kind representing an decimal integer.
    Integer,
    /// The token kind representing a floating point decimal number.
    Float,
    /// The token kind representing an operator (e.g. '+' in 1 + 2)
    Operator {operator: Operator},
    /// The token kind representing a bracket (e.g. '{', '}')
    Bracket {bracket: Bracket},
    /// The token kind representing a comma separator.
    Comma,
    /// The token kind representing a period, used in attribute accessors.
    Dot
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::Unknown
    }
}

use kaleidoscope_macro::impl_display;
impl_display!(TokenKind);
