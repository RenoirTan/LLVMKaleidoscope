use std::cell::RefCell;
use serde::{Serialize, Serializer, ser::SerializeSeq};
use super::Tokenizer;

pub struct ImmutableTokenizer {
    tokenizer: RefCell<Tokenizer>
}

impl ImmutableTokenizer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {tokenizer: RefCell::new(tokenizer)}
    }
}

impl Serialize for ImmutableTokenizer {
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
