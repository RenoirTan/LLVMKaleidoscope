//! A struct that reads a file and creates tokens from them.
//! 
//! See [`Tokenizer`].

use std::iter::Iterator;
use super::{FileStream, TokenIterator, LexerTupleRef, LexerTupleMut};
use crate::{
    error::Result,
    token::Token,
    utils
};

/// The tokeniser which iterates over the characters in a file stream and
/// yields a stream of tokens.
pub struct Tokenizer {
    pub last_unit: Option<char>
}

impl Tokenizer {
    /// Create a new [`Tokenizer`].
    pub fn new() -> Self {
        Self { last_unit: None }
    }

    /// See if any more tokens are available.
    #[inline]
    pub fn is_done(&self, stream: &FileStream) -> bool {
        stream.eof_reached()
    }

    /// Convert this tokenizer into an iterator.
    pub fn to_iter(self, stream: FileStream) -> TokenIterator {
        TokenIterator::new(stream, self)
    }

    pub fn to_tuple_ref<'a>(
        &'a self,
        stream: &'a FileStream
    ) -> LexerTupleRef<'a> {
        LexerTupleRef(stream, self)
    }

    pub fn to_tuple_mut<'a>(
        &'a mut self,
        stream: &'a mut FileStream
    ) -> LexerTupleMut<'a> {
        LexerTupleMut(stream, self)
    }

    /// Get the next token by reading from a file stream.
    pub fn next_token(&mut self, stream: &mut FileStream) -> Result<Token> {
        if stream.eof_reached() {
            return Ok(Token::new_eof(stream.get_index()));
        }
        if self.last_unit.is_none() {
            self.last_unit = match stream.next() {
                Some(u) => Some(u),
                None => {
                    return match stream.get_err() {
                        None => Ok(Token::new_eof(stream.get_index())),
                        Some(e) => Err(e)
                    };
                }
            }
        }
        let mut token = Token::default();
        let mut is_comment = false;
        'stream: loop {
            let index = stream.get_index();
            // None case already handled above.
            let unit = self.last_unit.unwrap();
            if is_comment {
                if utils::is_eol(unit) {
                    is_comment = false;
                }
            } else if utils::is_comment(unit) {
                is_comment = true;
            } else {
                match token.add_unit(unit, index) {
                    Ok(true) => {
                        token.end = index;
                        break 'stream;
                    },
                    Ok(false) => {},
                    Err(e) => return Err(e)
                }
            }
            self.last_unit = match stream.next() {
                Some(u) => Some(u),
                None => {
                    match stream.get_err() {
                        None => {
                            if token.is_empty() && stream.eof_reached() {
                                token = Token::new_eof(index);
                            } else {
                                token.resolve(index)?;
                            }
                            break 'stream;
                        },
                        Some(e) => return Err(e)
                    }
                }
            };
        }
        Ok(token)
    }
}
