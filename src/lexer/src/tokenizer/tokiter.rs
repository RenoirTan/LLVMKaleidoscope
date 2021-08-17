use std::iter::Iterator;
use crate::{
    error::Result,
    token::Token
};
use super::{FileStream, Tokenizer, LexerTupleRef, LexerTupleMut};

pub struct TokenIterator<'a> {
    stream: FileStream<'a>,
    tokenizer: Tokenizer,
    eof_count: usize
}

impl<'a> TokenIterator<'a> {
    pub fn new(stream: FileStream<'a>, tokenizer: Tokenizer) -> Self {
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

    pub fn to_tuple_ref(&'a self) -> LexerTupleRef<'a, 'a> {
        LexerTupleRef(&self.stream, &self.tokenizer)
    }

    pub fn to_tuple_mut(&'a mut self) -> LexerTupleMut<'a, 'a> {
        LexerTupleMut(&mut self.stream, &mut self.tokenizer)
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_count >= 1 {
            None
        } else {
            self.next_token().ok()
        }
    }
}
