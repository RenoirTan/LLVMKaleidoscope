use super::FileStream;
use crate::{
    error::Result,
    token::Token
};
use std::io::Read;

/// The tokeniser which iterates over the characters in a file stream and
/// yields a stream of tokens.
pub struct Tokenizer<S: Read> {
    pub stream: FileStream<S>,
}

impl<S: Read> Tokenizer<S> {
    pub fn new(stream: FileStream<S>) -> Self {
        Self { stream }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        if self.stream.eof_reached() {
            return Ok(Token::new_eof(self.stream.get_index()));
        }
        let mut token = Token::default();
        'stream: loop {
            let unit = match self.stream.next() {
                Some(u) => u,
                None => break 'stream
            };
            match token.add_unit(unit, self.stream.get_index()) {
                Ok(true) => break 'stream,
                Ok(false) => continue,
                Err(e) => return Err(e)
            }
        }
        Ok(token)
    }
}
