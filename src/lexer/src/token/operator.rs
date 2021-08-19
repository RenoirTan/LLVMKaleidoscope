//! A token representing an operator.
//! 
//! An operator is a symbol that acts like a function but has a different
//! syntax. Due to its significance, operators are given a special
//! representation in code.

use std::fmt;
use serde::{Serialize, Deserialize};

/// A token representing an operator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    /// A separate variant for inputs that is not a valid operator
    Unknown,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Multiply,
    /// /
    Divide,
    /// <
    LessThan,
    /// >
    GreaterThan,
    /// ==
    Equals,
    /// <=
    LessThanEqual,
    /// >=
    GreaterThanEqual,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.to_string())
    }
}

impl Operator {
    /// Convert a string representation of an operator to an [`Operator`] enum.
    /// If an invalid input is given, [`Operator::Unknown`] is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use kaleidoscope_lexer::token::Operator;
    /// 
    /// let plus = Operator::from_string("+");
    /// assert_eq!(plus, Operator::Plus);
    ///
    /// let unknown = Operator::from_string("bad");
    /// assert_eq!(unknown, Operator::Unknown);
    /// ```
    pub fn from_string(slice: &str) -> Self {
        use Operator::*;
        match slice {
            "+" => Plus,
            "-" => Minus,
            "*" => Multiply,
            "/" => Divide,
            "<" => LessThan,
            ">" => GreaterThan,
            "==" => Equals,
            "<=" => LessThanEqual,
            ">=" => GreaterThanEqual,
            _ => Unknown
        }
    }

    /// Convert this operator into a string representation of itself.
    ///
    /// # Example
    ///
    /// ```
    /// use kaleidoscope_lexer::token::Operator;
    ///
    /// let minus = Operator::Minus;
    /// assert_eq!(minus.to_string(), "-");
    /// ```
    pub fn to_string(&self) -> &'static str {
        use Operator::*;
        match self {
            Plus => "+",
            Minus => "-",
            Multiply => "*",
            Divide => "/",
            LessThan => "<",
            GreaterThan => ">",
            Equals => "==",
            LessThanEqual => "<=",
            GreaterThanEqual => ">=",
            Unknown => "???"
        }
    }
}
