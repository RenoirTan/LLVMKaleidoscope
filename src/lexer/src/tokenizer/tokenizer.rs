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
}

impl Tokenizer {
    pub fn new(stream: FileStream) -> Self {
        Self { stream }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        println!("[kaleidoscope_lexer::tokenizer::Tokenizer::next_token] called");
        if self.stream.eof_reached() {
            return Ok(Token::new_eof(self.stream.get_index()));
        }
        let mut token = Token::default();
        let mut is_comment = false;
        'stream: loop {
            let index = self.stream.get_index();
            let unit = match self.stream.next() {
                Some(u) => u,
                None => {
                    match self.stream.get_err() {
                        None => {
                            token.resolve(index)?;
                            break 'stream;
                        },
                        Some(e) => return Err(e)
                    }
                }
            };
            println!(
                "[kaleidoscope_lexer::tokenizer::Tokenizer::next_token] {:?} {}",
                unit,
                index
            );
            if is_comment {
                if utils::is_eol(unit) {
                    is_comment = false;
                }
            } else if utils::is_comment(unit) {
                is_comment = true;
            } else {
                match token.add_unit(unit, index) {
                    Ok(true) => break 'stream,
                    Ok(false) => {},
                    Err(e) => return Err(e)
                }
            }
        }
        Ok(token)
    }
}

impl Iterator for Tokenizer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().ok()
    }
}
