use std::iter::Iterator;
use crate::{
    error::Result,
    token::Token
};
use super::{FileStream, Tokenizer, LexerTupleRef, LexerTupleMut};

pub struct TokenIterator {
    stream: FileStream,
    tokenizer: Tokenizer,
    eof_count: usize
}

impl TokenIterator {
    pub fn new(stream: FileStream, tokenizer: Tokenizer) -> Self {
        Self {stream, tokenizer, eof_count: 0}
    }

    pub fn is_done(&self) -> bool {
        self.tokenizer.is_done(&self.stream)
    }

    pub fn eof_reached(&self) -> bool {
        self.eof_count >= 1
    }

    pub fn next_token(&mut self) -> Result<Token> {
        let token = self.tokenizer.next_token(&mut self.stream)?;
        if token.is_eof() {
            self.eof_count += 1;
        }
        Ok(token)
    }

    pub fn to_tuple_ref<'a>(&'a self) -> LexerTupleRef<'a> {
        LexerTupleRef(&self.stream, &self.tokenizer)
    }

    pub fn to_tuple_mut<'a>(&'a mut self) -> LexerTupleMut<'a> {
        LexerTupleMut(&mut self.stream, &mut self.tokenizer)
    }
}

impl Iterator for TokenIterator {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_count >= 1 {
            None
        } else {
            self.next_token().ok()
        }
    }
}
