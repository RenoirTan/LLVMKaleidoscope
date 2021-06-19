use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Keyword {
    Def,
    Extern
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
