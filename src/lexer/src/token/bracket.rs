use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt
};
use kaleidoscope_macro::impl_display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BracketKind {
    Round,
    Square,
    Curly,
    Angled
}

impl_display!(BracketKind);

impl BracketKind {
    pub fn from_int(code: i32) -> Option<Self> {
        Some(match code {
            0 => BracketKind::Round,
            1 => BracketKind::Square,
            2 => BracketKind::Curly,
            3 => BracketKind::Angled,
            _ => return None
        })
    }

    pub fn from_string(string: &str) -> Option<Self> {
        Some(match string {
            "(" | ")" => BracketKind::Round,
            "[" | "]" => BracketKind::Square,
            "{" | "}" => BracketKind::Curly,
            "<" | ">" => BracketKind::Angled,
            _ => return None
        })
    }

    pub(crate) fn get_repr(self) -> [&'static str; 2] {
        match self {
            BracketKind::Round => ["(", ")"],
            BracketKind::Square => ["[", "]"],
            BracketKind::Curly => ["{", "}"],
            BracketKind::Angled => ["<", ">"]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BracketSide {
    Left,
    Right
}

impl BracketSide {
    pub fn from_int(code: i32) -> Option<Self> {
        Some(match code {
            0 => BracketSide::Left,
            1 => BracketSide::Right,
            _ => return None
        })
    }

    pub fn from_string(string: &str) -> Option<Self> {
        Some(match string {
            "(" | "[" | "{" | "<" => BracketSide::Left,
            ")" | "]" | "}" | ">" => BracketSide::Right,
            _ => return None
        })
    }

    pub fn as_int(self) -> i32 {
        match self {
            BracketSide::Left => 0,
            BracketSide::Right => 1
        }
    }
}

impl Into<i32> for BracketSide {
    fn into(self) -> i32 {
        self.as_int()
    }
}

impl PartialOrd for BracketSide {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BracketSide {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_int().cmp(&other.as_int())
    }
}

impl_display!(BracketSide);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bracket {
    pub kind: BracketKind,
    pub side: BracketSide
}

impl fmt::Display for Bracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str_repr())
    }
}

impl Bracket {
    pub fn from_codes(kind: i32, side: i32) -> Option<Self> {
        BracketKind::from_int(kind)
            .zip(BracketSide::from_int(side))
            .map(|(kind, side)| Self {kind, side})
    }

    pub fn from_string(string: &str) -> Option<Self> {
        BracketKind::from_string(string)
            .zip(BracketSide::from_string(string))
            .map(|(kind, side)| Self {kind, side})
    }

    /// Get the bracket as a string
    pub fn str_repr(&self) -> &'static str {
        // Can be unwrapped safely as BracketSide::as_int
        // only returns 0 or 1.
        // BracketKind::get_repr returns an array of length 2.
        *self.kind
            .get_repr()
            .get(self.side.as_int() as usize)
            .unwrap()
    }

    /// Checks if 2 brackets are of the same type.
    /// 
    /// In addition, the first bracket must be the left bracket and the
    /// second (or `other`) bracket must be the right bracket in order for
    /// both brackets to cancel out.
    /// Otherwise this method will return false, marking both brackets as
    /// incompatible with each other.
    /// 
    /// # Example
    /// 
    /// ```
    /// let round_left = Bracket {
    ///     kind: BracketKind::Round,
    ///     side: BracketSide::Left
    /// };
    /// let round_right = Bracket {
    ///     kind: BracketKind::Round,
    ///     side: BracketSide::Right
    /// };
    /// assert!(round_left.cancels_out(round_right));
    /// 
    /// let square_left = Bracket {
    ///     kind: BracketKind::Square,
    ///     side: BracketKind::Left
    /// };
    /// assert!(!square_left.cancels_out(round_right));
    /// ```
    pub fn cancels_out(self, other: Self) -> bool {
        self.kind == other.kind && self.side < other.side
    }
}
