//! Types which represent a token in a Kaleidoscope program.
//! These structures describe the various aspects a token has, including the
//! kind of token it is and where it is found.

mod bracket;
mod fileindex;
mod keyword;
mod operator;
mod token;
mod tokenkind;

pub use bracket::{BracketKind, BracketSide, Bracket, brackets::{self, *}};
pub use fileindex::FileIndex;
pub use keyword::Keyword;
pub use operator::Operator;
pub use token::Token;
pub use tokenkind::TokenKind;
