//! A token representing an operator.

use kaleidoscope_macro::impl_display;

/// A token representing an operator.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Unknown,
    Plus,
    Minus,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equals,
    LessThanEqual,
    GreaterThanEqual,
}

impl_display!(Operator);

impl Operator {
    /// Convert a string representation of an operator to an [`Operator`] enum.
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
}
