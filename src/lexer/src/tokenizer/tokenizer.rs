//! A struct that reads a file and creates tokens from them.
//! 
//! See [`Tokenizer`].

use std::iter::Iterator;
use super::FileStream;
use crate::{
    error::Result,
    token::Token,
    utils
};

/// The tokeniser which iterates over the characters in a file stream and
/// yields a stream of tokens.
pub struct Tokenizer {
    pub stream: FileStream,
    pub last_unit: Option<char>
}

impl Tokenizer {
    /// Create a new [`Tokenizer`] from a [`FileStream`].
    pub fn new(stream: FileStream) -> Self {
        Self { stream, last_unit: None }
    }

    /// Get the next token by reading from the file stream.
    pub fn next_token(&mut self) -> Result<Token> {
        if self.stream.eof_reached() {
            return Ok(Token::new_eof(self.stream.get_index()));
        }
        if self.last_unit.is_none() {
            self.last_unit = match self.stream.next() {
                Some(u) => Some(u),
                None => {
                    return match self.stream.get_err() {
                        None => Ok(Token::new_eof(self.stream.get_index())),
                        Some(e) => Err(e)
                    };
                }
            }
        }
        let mut token = Token::default();
        let mut is_comment = false;
        'stream: loop {
            let index = self.stream.get_index();
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
            self.last_unit = match self.stream.next() {
                Some(u) => Some(u),
                None => {
                    match self.stream.get_err() {
                        None => {
                            if token.is_empty() && self.stream.eof_reached() {
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

impl Iterator for Tokenizer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stream.eof_reached() {
            None
        } else {
            self.next_token().ok()
        }
    }
}
