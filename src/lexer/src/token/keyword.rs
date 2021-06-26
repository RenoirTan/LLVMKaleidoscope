/// An enumerator of possible keywords that can be encountered in Kaleidoscope.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    /// `def` keyword. Define a function.
    Def,
    /// `extern` keyword. For foreign function interfaces.
    Extern,
    /// `if` keyword. Control flow.
    If,
    /// `else` keyword. Control flow.
    Else,
    /// `then` keyword. Control flow.
    Then,
}

impl Keyword {
    pub fn from_string(string: &str) -> Option<Self> {
        Some(match string {
            "def" => Keyword::Def,
            "extern" => Keyword::Extern,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "then" => Keyword::Then,
            _ => return None
        })
    }
}

use kaleidoscope_macro::impl_display;
impl_display!(Keyword);
