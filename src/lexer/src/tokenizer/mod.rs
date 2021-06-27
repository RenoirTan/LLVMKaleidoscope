//! The tokeniser which spits out tokens from a Kaleidoscope file.
//!
//! See also [`crate::tokenizer::Tokenizer`]

mod filestream;
mod immuttokenizer;
mod tokenizer;

pub use filestream::FileStream;
pub use immuttokenizer::ImmutableTokenizer;
pub use tokenizer::Tokenizer;
