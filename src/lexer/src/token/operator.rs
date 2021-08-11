//! A token representing an operator.

use std::fmt;
use serde::{Serialize, Deserialize};

/// A token representing an operator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.to_string())
    }
}

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
