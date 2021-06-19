use std::fmt;
use super::Keyword;

#[derive(Copy, Clone, Debug)]
pub enum TokenKind {
    Unknown,
    Eof,
    Keyword {keyword: Keyword},
    Identifier,
    Integer,
    Float
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
