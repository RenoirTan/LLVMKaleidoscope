//! The tokeniser which spits out tokens from a Kaleidoscope file.
//!
//! See also [`crate::tokenizer::Tokenizer`]

mod filestream;
mod lexerser;
mod tokenizer;

pub use filestream::FileStream;
pub use lexerser::LexerSerializer;
pub use tokenizer::Tokenizer;
