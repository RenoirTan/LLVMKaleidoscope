use kaleidoscope_macro::impl_display;

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
