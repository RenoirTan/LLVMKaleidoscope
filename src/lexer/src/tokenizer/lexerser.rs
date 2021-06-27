//! A module for serialising a tokeniser.

use std::cell::RefCell;
use serde::{Serialize, Serializer, ser::SerializeSeq};
use super::Tokenizer;

/// Serialises a [`Tokenizer`] into a list of tokens.
/// 
/// Can be used to store tokens into a different format for later use.
pub struct LexerSerializer {
    tokenizer: RefCell<Tokenizer>
}

impl LexerSerializer {
    /// Create a new tokeniser serialiser.
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {tokenizer: RefCell::new(tokenizer)}
    }
}

impl Serialize for LexerSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_seq(None)?;
        while let Some(token) = self.tokenizer.borrow_mut().next() {
            state.serialize_element(&token)?;
        }
        state.end()
    }
}
