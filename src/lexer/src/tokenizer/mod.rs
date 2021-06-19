//! The tokeniser which spits out tokens from a Kaleidoscope file.
//! 
//! See also [`crate::tokenizer::Tokenizer`]

mod stream;
mod tokenizer;

pub use stream::Stream;
pub use tokenizer::Tokenizer;
