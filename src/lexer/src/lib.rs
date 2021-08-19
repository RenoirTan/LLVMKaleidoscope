//! This library contains utilities which converts a stream of (currently only
//! UTF-8) bytes to a stream of tokens. This is particularly helpful when
//! trying to simplify the parsing process of a Kaleidoscope program by
//! separating the parsing process into 2 steps, with this library responsible
//! for the first step.

pub mod error;
pub mod token;
pub mod tokenizer;
pub mod utils;
