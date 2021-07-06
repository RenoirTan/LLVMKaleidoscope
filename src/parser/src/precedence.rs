use kaleidoscope_macro::impl_display;
use kaleidoscope_ast::nodes::Operator;

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
    pub fn from_operator(operator: Operator) -> Self {
        use BinaryOperatorPrecedence::*;
        match operator {
            Operator::Unknown => Unknown,
            Operator::GreaterThan
            | Operator::GreaterThanEqual
            | Operator::LessThan
            | Operator::LessThanEqual
            | Operator::Equals => Comparison,
            Operator::Plus
            | Operator::Minus => Addition,
            Operator::Multiply
            | Operator::Divide => Multiplication
        }
    }

    pub fn from_string(slice: &str) -> Self {
        Self::from_operator(Operator::from_string(slice))
    }

    pub fn get_lowest() -> Self {
        Self::Unknown
    }

    pub fn get_highest() -> Self {
        Self::Exponentiation
    }
}

impl Into<BinaryOperatorPrecedence> for Operator {
    fn into(self) -> BinaryOperatorPrecedence {
        return BinaryOperatorPrecedence::from_operator(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precedence() {
        let bad = BinaryOperatorPrecedence::from_string("fffirj");
        let plus = BinaryOperatorPrecedence::from_string("+");
        let multiply = BinaryOperatorPrecedence::from_string("*");
        assert!(bad < plus);
        assert!(plus < multiply);
    }
}
