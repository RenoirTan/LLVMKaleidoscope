use super::FileStream;
use crate::{
    error::{Error, ErrorKind, Result},
    token::{Keyword, Token, TokenKind},
    utils,
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
        token.start = self.stream.get_index();
        'stream: for last_unit in &mut self.stream {
            if token.span.len() == 0 {
                if utils::is_whitespace(last_unit) {
                    continue 'stream;
                } else if utils::is_alpha(last_unit) {
                    token.token_kind = TokenKind::Identifier;
                } else if utils::is_decimal_digit(last_unit) {
                    token.token_kind = TokenKind::Integer;
                } else if utils::is_fullstop(last_unit) {
                    token.token_kind = TokenKind::Float;
                } else {
                    return Err(Error::new(
                        &format!(
                            "Unknown character: {} at index {}",
                            last_unit,
                            self.stream.get_index()
                        ),
                        ErrorKind::InvalidChar,
                        None
                    ));
                }
                token.span.push(last_unit);
            } else {
                if utils::is_whitespace(last_unit) {
                    break 'stream;
                } else if utils::is_alphanum(last_unit) {
                    token.span.push(last_unit);
                } else if utils::is_fullstop(last_unit) {
                    match token.token_kind {
                        TokenKind::Integer => {
                            token.span.push(last_unit);
                            token.token_kind = TokenKind::Float;
                        }
                        TokenKind::Float => {
                            return Err(Error::new(
                                &format!(
                                    "Bad character: {} at index {}",
                                    last_unit,
                                    self.stream.get_index()
                                ),
                                ErrorKind::BadChar,
                                None
                            ));
                        }
                        _ => break 'stream,
                    }
                }
            }
        }
        token.end = self.stream.get_index();
        match token.token_kind {
            TokenKind::Unknown => Err(Error::new(
                &format!(
                    "Invalid token:\n\t{}\nat index {}",
                    token.span,
                    token.start
                ),
                ErrorKind::InvalidToken,
                None
            )),
            TokenKind::Eof => Ok(token),
            TokenKind::Identifier => match &token.span[..] {
                "def" => {
                    token.token_kind = TokenKind::Keyword {
                        keyword: Keyword::Def,
                    };
                    Ok(token)
                }
                "extern" => {
                    token.token_kind = TokenKind::Keyword {
                        keyword: Keyword::Extern,
                    };
                    Ok(token)
                }
                _ => Ok(token),
            },
            TokenKind::Float => match <dyn AsRef<[u8]>>::as_ref(&token.span)
                .last()
            {
                Some(unit) if *unit == '.' as u8 => {
                    Err(
                        Error::new(
                            &format!(
                                "No fractional part after decimal point at index {}",
                                token.end
                            ),
                            ErrorKind::LexerFatal,
                            None
                        )
                    )
                },
                Some(_) => {
                    Ok(token)
                },
                None => {
                    Err(
                        Error::new(
                            &"Empty string treated as float by lexer...",
                            ErrorKind::LexerFatal,
                            None
                        )
                    )
                }
            },
            _ => Ok(token)
        }
    }
}
