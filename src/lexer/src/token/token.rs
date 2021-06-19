use super::{TokenKind, FileIndex};

/// A token in a Kaleidoscope file.
/// 
/// This structural representation of a token contains the
/// possible `TokenKind` of the token,
/// the token as a string (stored as `span`),
/// as well as the start and end indices of the token.
#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub span: String,
    pub start: FileIndex,
    pub end: FileIndex
}

use kaleidoscope_macro::impl_display;
impl_display!(Token);

impl Token {
    pub fn new(
        token_kind: TokenKind,
        span: String,
        start: FileIndex,
        end: FileIndex
    ) -> Self {
        Self {token_kind, span, start, end}
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_kind: TokenKind::Unknown,
            span: String::new(),
            start: Default::default(),
            end: Default::default()
        }
    }
}
