//! A bunch of types representing a bracket such as `(` or `]`.
//! Objects of these types can also check to see if they match or cancel
//! out each other, which can make sure that the brackets in a programme are
//! balanced out properly.

use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt
};

use kaleidoscope_macro::impl_display;
use serde::{Deserialize, Serialize};

pub mod brackets {
    use super::{Bracket, BracketKind, BracketSide};

    pub const LEFT_ROUND_BRACKET: Bracket = Bracket {
        kind: BracketKind::Round,
        side: BracketSide::Left
    };
    pub const RIGHT_ROUND_BRACKET: Bracket = Bracket {
        kind: BracketKind::Round,
        side: BracketSide::Right
    };
    pub const LEFT_SQUARE_BRACKET: Bracket = Bracket {
        kind: BracketKind::Square,
        side: BracketSide::Left
    };
    pub const RIGHT_SQUARE_BRACKET: Bracket = Bracket {
        kind: BracketKind::Square,
        side: BracketSide::Right
    };
    pub const LEFT_CURLY_BRACKET: Bracket = Bracket {
        kind: BracketKind::Curly,
        side: BracketSide::Left
    };
    pub const RIGHT_CURLY_BRACKET: Bracket = Bracket {
        kind: BracketKind::Curly,
        side: BracketSide::Right
    };
    pub const LEFT_ANGLED_BRACKET: Bracket = Bracket {
        kind: BracketKind::Angled,
        side: BracketSide::Left
    };
    pub const RIGHT_ANGLED_BRACKET: Bracket = Bracket {
        kind: BracketKind::Angled,
        side: BracketSide::Right
    };
    pub const UNKNOWN_BRACKET: Bracket = Bracket {
        kind: BracketKind::Unknown,
        side: BracketSide::Left
    };
}

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
    /// fn is_in_correct_order(first: BracketSide, second: BracketSide) -> bool {
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

    /// As the name of this method suggests, it checks whether the
    /// [`BracketSide`] matches [`BracketSide::Left`].
    #[inline]
    pub fn is_left(&self) -> bool {
        matches!(self, &BracketSide::Left)
    }

    /// Check whether the [`BracketSide`] matches [`BracketSide::Right`].
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
        Self { kind, side }
    }

    /// Create a [`Bracket`] from a string of a bracket.
    /// If the string does not contain a valid bracket, [`Bracket::kind`]
    /// becomes [`BracketKind::Unknown`] and [`Bracket::side`] resets to
    /// [`BracketSide::Left`] by default.
    ///
    /// # Example
    ///
    /// ```
    /// use kaleidoscope_lexer::token::{Bracket, BracketKind};
    ///
    /// let lr_bracket = Bracket::from_string("(");
    /// assert!(matches!(lr_bracket.kind, BracketKind::Round));
    /// assert!(lr_bracket.side.is_left());
    ///
    /// let bad_bracket = Bracket::from_string("clearly not a bracket");
    /// assert!(matches!(bad_bracket.kind, BracketKind::Unknown));
    /// assert!(bad_bracket.side.is_left());
    /// ```
    pub fn from_string(string: &str) -> Self {
        let kind = BracketKind::from_string(string);
        let side = BracketSide::from_string(string).unwrap_or_default();
        Self { kind, side }
    }

    /// Return the bracket as a string representaton of itself.
    ///
    /// # Example
    ///
    /// ```
    /// use kaleidoscope_lexer::token::Bracket;
    ///
    /// let rs_bracket = Bracket::from_string("]");
    /// assert_eq!(rs_bracket.str_repr(), "]");
    /// ```
    pub fn str_repr(&self) -> &'static str {
        // Can be unwrapped safely as BracketSide::as_int
        // only returns 0 or 1.
        // BracketKind::get_repr returns an array of length 2.
        *self
            .kind
            .get_repr()
            .get(self.side.as_int() as usize)
            .unwrap()
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self.kind, BracketKind::Unknown)
    }

    /// Check if the string representation of a bracket matches a bracket
    /// represented internally using a [`Bracket`] object.
    ///
    /// If both `self` and the other bracket are unknown, this method will
    /// still return false for safety (like NaN !== NaN in JavaScript).
    ///
    /// # Example
    ///
    /// ```
    /// use kaleidoscope_lexer::token::{Bracket, BracketKind, BracketSide};
    ///
    /// let left_round = Bracket {
    ///     kind: BracketKind::Round,
    ///     side: BracketSide::Left
    /// };
    /// let unknown = Bracket {
    ///     kind: BracketKind::Unknown,
    ///     side: BracketSide::Left
    /// };
    ///
    /// assert!(left_round.is_str("("));
    /// assert!(!left_round.is_str("]"));
    ///
    /// // Although both `unknown` and "???" are both not brackets,
    /// // `false` is still returned.
    /// assert!(!unknown.is_str("???"));
    /// ```
    pub fn is_str(&self, string: &str) -> bool {
        let other = Bracket::from_string(string);
        if let BracketKind::Unknown = other.kind {
            false
        } else {
            *self == other
        }
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

impl<T: AsRef<str>> PartialEq<T> for Bracket {
    fn eq(&self, other: &T) -> bool {
        self.is_str(other.as_ref())
    }
}
