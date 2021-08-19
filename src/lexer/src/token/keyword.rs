//! A enum representing a keyword. A key reason why I chose to define a special
//! enum because manually checking if the span in a token is what I was
//! looking for matches a keyword.
//! 
//! See [`Keyword`].

use std::fmt;
use serde::{Serialize, Deserialize};

/// An enumerator of possible keywords that can be encountered in Kaleidoscope.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

    /// Convert the keyword enum into a string representation of itself.
    ///
    /// # Example
    ///
    /// ```
    /// use kaleidoscope_lexer::token::Keyword;
    ///
    /// assert_eq!(Keyword::from_string("def").unwrap().to_string(), "def");
    /// ```
    pub fn to_string(&self) -> &'static str {
        match *self {
            Keyword::Def => "def",
            Keyword::Extern => "extern",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::Then => "then"
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.to_string())
    }
}
