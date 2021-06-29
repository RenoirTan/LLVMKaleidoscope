//! A module for serialising a tokeniser.

use std::cell::RefCell;
use serde::{Serialize, Serializer, ser::SerializeSeq};
use super::{TokenIterator};

/// Serialises a [`TokenIterator`] into a list of tokens.
/// 
/// Can be used to store tokens into a different format for later use.
pub struct LexerSerializer {
    token_iter: RefCell<TokenIterator>
}

impl LexerSerializer {
    /// Create a new tokeniser serialiser.
    pub fn new(token_iter: TokenIterator) -> Self {
        Self {token_iter: RefCell::new(token_iter)}
    }
}

impl Serialize for LexerSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_seq(None)?;
        while let Some(token) = self.token_iter.borrow_mut().next() {
            state.serialize_element(&token)?;
        }
        state.end()
    }
}
