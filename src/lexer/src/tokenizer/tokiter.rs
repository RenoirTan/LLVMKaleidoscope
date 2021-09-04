//! A module defining a [`TokenIterator`] that can continually read a file
//! stream and spit out tokens in a for loop.

use std::iter::Iterator;

use super::{FileStream, LexerTupleMut, LexerTupleRef, Tokenizer};
use crate::{error::Result, token::Token};

/// A structure that takes a [`FileStream`] and reads the characters to produce
/// one token for each iteration in a for loop.
pub struct TokenIterator<'a> {
    stream:    FileStream<'a>,
    tokenizer: Tokenizer,
    eof_count: usize
}

impl<'a> TokenIterator<'a> {
    /// Create a new [`TokenIterator`].
    pub fn new(stream: FileStream<'a>, tokenizer: Tokenizer) -> Self {
        Self {
            stream,
            tokenizer,
            eof_count: 0
        }
    }

    /// Check if this iterator can produce more tokens.
    pub fn is_done(&self) -> bool {
        self.tokenizer.is_done(&self.stream)
    }

    /// Check if an EOF character has been encountered.
    pub fn eof_reached(&self) -> bool {
        self.eof_count >= 1
    }

    /// Get the next token in from the tokeniser.
    pub fn next_token(&mut self) -> Result<Token> {
        let token = self.tokenizer.next_token(&mut self.stream)?;
        if token.is_eof() {
            self.eof_count += 1;
        }
        Ok(token)
    }

    /// Convert this iterator into a neatly packaged tuple for transport
    /// across functions.
    pub fn to_tuple_ref(&'a self) -> LexerTupleRef<'a, 'a> {
        LexerTupleRef(&self.stream, &self.tokenizer)
    }

    /// Convert this iterator into a neatly packaged tuple for transport
    /// across functions.
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
