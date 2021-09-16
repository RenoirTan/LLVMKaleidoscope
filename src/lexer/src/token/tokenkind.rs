//! The type of the token. Such a classification allows the parser to make
//! sure that the input given is correct.
//!
//! See [`TokenKind`].

use serde::{Deserialize, Serialize};

use super::{Bracket, Keyword, Operator};

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
    Keyword(Keyword),
    /// The token kind representing an identifier.
    Identifier,
    /// The token kind representing an decimal integer.
    Integer,
    /// The token kind representing a floating point decimal number.
    Float,
    /// The token kind representing an operator (e.g. '+' in 1 + 2)
    Operator(Operator),
    /// The token kind representing a bracket (e.g. '{', '}')
    Bracket(Bracket),
    /// The token kind representing a comma separator.
    Comma,
    /// The token kind representing a period, used in attribute accessors.
    Dot,
    /// A semicolon denotes the end of a statement/expression
    Semicolon
}

impl TokenKind {
    /// Check whether this token kind represents a sentinel value that tells
    /// the parser to halt and return whatever it has processed. Such tokens
    /// include semicolons and EOFs.
    pub fn is_terminating(&self) -> bool {
        matches!(*self, TokenKind::Semicolon | TokenKind::Eof)
    }
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::Unknown
    }
}

use kaleidoscope_macro::impl_display;
impl_display!(TokenKind);
