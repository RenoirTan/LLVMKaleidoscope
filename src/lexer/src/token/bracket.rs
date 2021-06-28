//! A token kind representing a bracket.

use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt
};
use serde::{Serialize, Deserialize};
use kaleidoscope_macro::impl_display;

/// The type of bracket [`Bracket`] represents.
/// This merely classifies a bracket by its shape
/// (i.e. whether it's round, square or curly, etc.)
/// and not its orientation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BracketKind {
    Unknown,
    Round,
    Square,
    Curly,
    Angled
}

impl_display!(BracketKind);

impl BracketKind {
    /// Convert an integer code to a [`BracketKind`].
    pub fn from_int(code: i32) -> Self {
        match code {
            0 => BracketKind::Round,
            1 => BracketKind::Square,
            2 => BracketKind::Curly,
            3 => BracketKind::Angled,
            _ => BracketKind::Unknown
        }
    }

    /// Convert the string representation of a bracket to a [`BracketKind`].
    /// 
    /// # Example
    /// 
    /// An example showing a valid bracket being converted to a [`BracketKind`].
    /// 
    /// ```
    /// use kaleidoscope_lexer::token::BracketKind;
    /// 
    /// let bracket_kind = BracketKind::from_string("[");
    /// assert!(matches!(bracket_kind, BracketKind::Square));
    /// ```
    /// 
    /// If a string that does not represent a bracket is passed to this
    /// function, [`BracketKind::Unknown`] is returned.
    /// 
    /// ```
    /// use kaleidoscope_lexer::token::BracketKind;
    /// 
    /// let bracket_kind = BracketKind::from_string("invalid");
    /// assert!(matches!(bracket_kind, BracketKind::Unknown));
    /// ```
    pub fn from_string(string: &str) -> Self {
        match string {
            "(" | ")" => BracketKind::Round,
            "[" | "]" => BracketKind::Square,
            "{" | "}" => BracketKind::Curly,
            "<" | ">" => BracketKind::Angled,
            _ => BracketKind::Unknown
        }
    }

    /// Get the string representation of each type of bracket as an array of
    /// string slices. This array has a predetermined length of 2, with
    /// the first element representing the left side of that type of bracket
    /// while the second one is the right side.
    /// 
    /// The string representation for [`BracketKind::Unknown`] is "??" for both
    /// sides.
    /// 
    /// # Example
    /// 
    /// ```compile_fail
    /// use kaleidoscope_lexer::token::BracketKind;
    /// 
    /// assert_eq!(BracketKind::Curly.get_repr(), ["{", "}"]);
    /// ```
    pub(crate) fn get_repr(self) -> [&'static str; 2] {
        match self {
            BracketKind::Round => ["(", ")"],
            BracketKind::Square => ["[", "]"],
            BracketKind::Curly => ["{", "}"],
            BracketKind::Angled => ["<", ">"],
            BracketKind::Unknown => ["??", "??"]
        }
    }
}

/// The orientation or side of a bracket. This can either be left or right.
/// Brackets which are classified as "Left" include "(", "[" and "{", etc, with
/// their corresponding counterparts (")", "]", "}" respectively) are counted
/// as "Right"-side brackets.
/// 
/// In order to compare brackets by their side, this enum can be converted
/// to an integer value, with [`BracketSide::Left`]'s value being 0 and
/// [`BracketSide::Right`]'s being 1. This is useful when you want to make
/// sure that 2 brackets are in the correct order.
/// See more: [`BracketSide::as_int`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BracketSide {
    Left,
    Right
}

impl BracketSide {
    /// Convert an integer to a [`BracketSide`].
    /// 
    /// 0 is Left and
    /// 1 is Right.
    pub fn from_int(code: i32) -> Option<Self> {
        Some(match code {
            0 => BracketSide::Left,
            1 => BracketSide::Right,
            _ => return None
        })
    }

    /// Convert the string representation of a bracket to a [`BracketSide`].
    pub fn from_string(string: &str) -> Option<Self> {
        Some(match string {
            "(" | "[" | "{" | "<" => BracketSide::Left,
            ")" | "]" | "}" | ">" => BracketSide::Right,
            _ => return None
        })
    }

    /// Convert a [`BracketSide`] to an integer. This can be used to check if
    /// 2 brackets are in the correct order. For example, if the first
    /// bracket in a sequence is 0 (i.e. left) and the second bracket in the
    /// sequence is 1 (i.e. right), then you can be certain that both brackets
    /// are in the correct order.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kaleidoscope_lexer::token::BracketSide;
    /// 
    /// fn is_in_correct_order(
    ///     first: BracketSide,
    ///     second: BracketSide
    /// ) -> bool {
    ///     first.as_int() < second.as_int()
    /// }
    /// 
    /// assert!(is_in_correct_order(
    ///     BracketSide::from_string("[").unwrap(),
    ///     BracketSide::from_string("]").unwrap()
    /// ));
    /// 
    /// assert!(!is_in_correct_order(
    ///     BracketSide::from_string("}").unwrap(),
    ///     BracketSide::from_string("{").unwrap()
    /// ));
    /// ```
    pub fn as_int(self) -> i32 {
        match self {
            BracketSide::Left => 0,
            BracketSide::Right => 1
        }
    }

    #[inline]
    pub fn is_left(&self) -> bool {
        matches!(self, &BracketSide::Left)
    }

    #[inline]
    pub fn is_right(&self) -> bool {
        matches!(self, &BracketSide::Right)
    }
}

impl Default for BracketSide {
    fn default() -> Self {
        BracketSide::Left
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

/// A struct representing a bracket. This bracket can be grouped by its
/// kind and side (see [`BracketKind`] and [`BracketSide`] respectively).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Create a bracket from the corresponding [`BracketKind`] and
    /// [`BracketSide`] integer codes.
    pub fn from_codes(kind: i32, side: i32) -> Self {
        let kind = BracketKind::from_int(kind);
        let side = BracketSide::from_int(side).unwrap_or_default();
        Self {kind, side}
    }

    /// Create a [`Bracket`] from a string of a bracket.
    pub fn from_string(string: &str) -> Self {
        let kind = BracketKind::from_string(string);
        let side = BracketSide::from_string(string).unwrap_or_default();
        Self {kind, side}
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
    /// use kaleidoscope_lexer::token::{Bracket, BracketKind, BracketSide};
    /// 
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
    ///     side: BracketSide::Left
    /// };
    /// assert!(!square_left.cancels_out(round_right));
    /// ```
    pub fn cancels_out(self, other: Self) -> bool {
        self.kind == other.kind && self.side < other.side
    }
}
