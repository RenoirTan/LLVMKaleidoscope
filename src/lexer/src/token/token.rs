use std::fmt;
use super::{TokenKind, Index};

#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub span: String,
    pub start: Index
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}