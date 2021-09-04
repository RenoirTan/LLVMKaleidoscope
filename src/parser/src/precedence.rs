//! A module for managing the precedence of binary operators.
//!
//! In math, there are conventions specifying which part of an expression to
//! evaluate first. Given that modern high-level programming languages draw
//! heavy inspiration from math for their syntax, we have to make sure that
//! the expressions inputted into the parser are sorted in the correct
//! order. For Kaleidoscope, the rule to follow is *BODMAS/PEMDAS*.
//!
//! To allow the parser to organise the operators in an expression, we must
//! give each operator a ranking, or a so-called "precedence". Values
//! near an operator with a larger precedence will coalesce around that
//! more important operator rather than an adjacent operator of a lower
//! priority. Such an appointment allows the parser to "understand" the
//! expression unambiguously to the user's expectations.

use kaleidoscope_ast::nodes::Operator;
use kaleidoscope_macro::impl_display;

/// An enumerator defining the precedence of each operator. Operators are
/// categorised into a smaller number of actions, such as "Comparison",
/// "Addition", "Multiplication" and "Exponentiation".
///
/// These actions can be compared with each other by their priority.
/// For example, multiplication operations (* or /) have a higher precedence
/// than addition operations (+ or -), so doing
///
/// ```compile_fail
/// BinaryOperatorPrecedence::from_string("*") > BinaryOperatorPrecedence::from_string("-")
/// ```
///
/// evaluates to `true`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperatorPrecedence {
    Unknown,
    Comparison,
    Addition,
    Multiplication,
    Exponentiation
}

impl_display!(BinaryOperatorPrecedence);

impl BinaryOperatorPrecedence {
    /// Get the operator precedence of an operator.
    pub fn from_operator(operator: Operator) -> Self {
        use BinaryOperatorPrecedence::*;
        match operator {
            Operator::Unknown => Unknown,
            Operator::GreaterThan
            | Operator::GreaterThanEqual
            | Operator::LessThan
            | Operator::LessThanEqual
            | Operator::Equals => Comparison,
            Operator::Plus | Operator::Minus => Addition,
            Operator::Multiply | Operator::Divide => Multiplication
        }
    }

    /// Get the operator precedence from the string version of the
    /// operator.
    pub fn from_string(slice: &str) -> Self {
        Self::from_operator(Operator::from_string(slice))
    }

    /// Get the precedence variant with the lowest priority.
    ///
    /// In this case, it's [`BinaryOperatorPrecedence::Unknown`].
    pub fn get_lowest() -> Self {
        Self::Unknown
    }

    /// Get the precedence variant with the highest priority.
    ///
    /// In this case, it's [`BinaryOperatorPrecedence::Exponentiation`].
    pub fn get_highest() -> Self {
        Self::Exponentiation
    }
}

impl Into<BinaryOperatorPrecedence> for Operator {
    fn into(self) -> BinaryOperatorPrecedence {
        return BinaryOperatorPrecedence::from_operator(self);
    }
}
