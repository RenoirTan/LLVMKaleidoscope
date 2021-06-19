use super::Keyword;

/// The type of token a token is.
/// 
/// That's the worst explanation I have ever written in my life.
#[derive(Copy, Clone, Debug)]
pub enum TokenKind {
    Unknown,
    Eof,
    Keyword {keyword: Keyword},
    Identifier,
    Integer,
    Float
}

use kaleidoscope_macro::impl_display;
impl_display!(TokenKind);
