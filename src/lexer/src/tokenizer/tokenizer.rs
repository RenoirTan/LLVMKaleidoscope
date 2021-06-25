use super::FileStream;
use crate::{error::{Error, ErrorKind, Result}, token::Token, utils};

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
        if self.stream.eof_reached() {
            return Ok(Token::new_eof(self.stream.get_index()));
        }
        let mut unit = match self.stream.get_unit() {
            Some(u) => u,
            None => return Err(Error::new(
                &format!("Could not read first character in file/stream."),
                ErrorKind::FileIOError,
                None
            ))
        };
        let mut token = Token::default();
        let mut is_comment = false;
        'stream: loop {
            if is_comment {
                if utils::is_eol(unit) {
                    is_comment = false;
                }
            } else if utils::is_comment(unit) {
                is_comment = true;
            } else {
                match token.add_unit(unit, self.stream.get_index()) {
                    Ok(true) => break 'stream,
                    Ok(false) => {},
                    Err(e) => return Err(e)
                }
            }
            unit = match self.stream.next() {
                Some(u) => u,
                None => {
                    match self.stream.get_err() {
                        None => {
                            token.resolve(self.stream.get_index())?;
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
