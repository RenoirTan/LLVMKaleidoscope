//! A keyword token.
//! 
//! See [`Keyword`].

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
    /// Convert a string to a variant of the [`Keyword`] enum.
    /// 
    /// If the string is not a valid keyword, [`None`] is returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kaleidoscope_lexer::token::Keyword;
    /// 
    /// assert!(matches!(
    ///     Keyword::from_string("extern"),
    ///     Some(Keyword::Extern)
    /// ));
    /// 
    /// assert!(matches!(
    ///     Keyword::from_string("not a keyword"),
    ///     None
    /// ));
    /// ```
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
