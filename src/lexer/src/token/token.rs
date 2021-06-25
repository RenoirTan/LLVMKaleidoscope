use crate::{
    error::{Error, ErrorKind, Result},
    utils
};
use super::{
    FileIndex,
    TokenKind,
    Keyword,
    Operator
};

/// A token in a Kaleidoscope file.
///
/// This structural representation of a token contains the
/// possible `TokenKind` of the token,
/// the token as a string (stored as `span`),
/// as well as the start and end indices of the token.
#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub span: String,
    pub start: FileIndex,
    pub end: FileIndex,
}

use kaleidoscope_macro::impl_display;
impl_display!(Token);

impl Token {
    /// Generate a new token from known values.
    pub fn new(
        token_kind: TokenKind,
        span: String,
        start: FileIndex,
        end: FileIndex
    ) -> Self {
        Self {
            token_kind,
            span,
            start,
            end,
        }
    }

    /// A new token whose [`TokenKind`] is [`TokenKind::Eof`].
    pub fn new_eof(index: FileIndex) -> Self {
        Self {
            token_kind: TokenKind::Eof,
            span: String::new(),
            start: index,
            end: index,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    pub fn add_unit(&mut self, unit: char, index: FileIndex) -> Result<bool> {
        if self.is_empty() {
            self.add_unit_when_empty(unit, index)
        } else {
            self.add_unit_when_not_empty(unit, index)
        }
    }

    fn add_unit_when_empty(
        &mut self,
        unit: char,
        index: FileIndex
    ) -> Result<bool> {
        self.start = index;
        if utils::is_whitespace(unit) {
            return Ok(false);
        } else if utils::is_alpha(unit) {
            self.token_kind = TokenKind::Identifier;
            self.span.push(unit);
        } else if utils::is_decimal_digit(unit) {
            self.token_kind = TokenKind::Integer;
            self.span.push(unit);
        } else if utils::is_opchar(unit) {
            self.span.push(unit);
            let operator = Operator::from_str(self.borrow_span());
            self.token_kind = TokenKind::Operator {operator};
        } else {
            return Err(Error::new(
                &format!("Invalid char {} at {}", unit, index),
                ErrorKind::InvalidChar,
                None
            ));
        }
        Ok(false)
    }

    fn add_unit_when_not_empty(
        &mut self,
        unit: char,
        index: FileIndex
    ) -> Result<bool> {
        if utils::is_whitespace(unit) {
            return self.resolve(index);
        }
        match self.token_kind {
            TokenKind::Identifier => self.add_unit_if_identifier(unit, index),
            TokenKind::Integer => self.add_unit_if_integer(unit, index),
            TokenKind::Float => self.add_unit_if_float(unit, index),
            TokenKind::Operator {..} => self.add_unit_if_operator(unit, index),
            _ => Err(Error::new(
                &format!(
                    "Uncaught TokenKind {} at {}",
                    self.token_kind,
                    index
                ),
                ErrorKind::LexerFatal,
                None
            ))
        }
    }

    fn add_unit_if_identifier(
        &mut self,
        unit: char,
        index: FileIndex
    ) -> Result<bool> {
        if utils::is_alphanum(unit) {
            self.span.push(unit);
            Ok(false)
        } else {
            self.resolve(index)
        }
    }

    fn add_unit_if_integer(
        &mut self,
        unit: char,
        index: FileIndex
    ) -> Result<bool> {
        if utils::is_decimal_digit(unit) {
            self.span.push(unit);
            Ok(false)
        } else if utils::is_fullstop(unit) {
            self.span.push(unit);
            self.token_kind = TokenKind::Float;
            Ok(false)
        } else {
            Err(Error::new(
                &format!("Bad char {} at {}", unit, index),
                ErrorKind::BadChar,
                None
            ))
        }
    }

    fn add_unit_if_float(
        &mut self,
        unit: char,
        index: FileIndex
    ) -> Result<bool> {
        if utils::is_decimal_digit(unit) {
            self.span.push(unit);
            Ok(false)
        } else {
            Err(Error::new(
                &format!("Bad char {} at {}", unit, index),
                ErrorKind::BadChar,
                None
            ))
        }
    }

    fn add_unit_if_operator(
        &mut self,
        _unit: char,
        index: FileIndex
    ) -> Result<bool> {
        Err(Error::new(
            &format!(
                "Kaleidoscope currently only accepts 1-character operands. Error at {}",
                index
            ),
            ErrorKind::ExcessiveChars,
            None
        ))
    }

    pub fn resolve(&mut self, index: FileIndex) -> Result<bool> {
        self.end = index;
        match self.token_kind {
            TokenKind::Unknown => Err(Error::new(
                &format!(
                    "Could not guess TokenKind from span '{}' at index {}",
                    self.span,
                    index
                ),
                ErrorKind::InvalidToken,
                None
            )),
            TokenKind::Identifier => {
                match &self.span[..] {
                    "def" => self.token_kind = TokenKind::Keyword {
                        keyword: Keyword::Def
                    },
                    "extern" => self.token_kind = TokenKind::Keyword {
                        keyword: Keyword::Extern
                    },
                    _ => {}
                }
                Ok(true)
            },
            TokenKind::Float => {
                match <dyn AsRef<[u8]>>::as_ref(&self.span).last() {
                    None => Err(Error::new(
                        &format!(
                            "Lexer detected a float in an empty span at index {}",
                            index
                        ),
                        ErrorKind::LexerFatal,
                        None
                    )),
                    Some(unit) => if *unit == '.' as u8 {
                        Err(Error::new(
                            &format!(
                                "Float cannot end with floating point at index {}",
                                index
                            ),
                            ErrorKind::BadChar,
                            None
                        ))
                    } else {
                        Ok(true)
                    }
                }
            },
            _ => Ok(true)
        }
    }

    pub fn borrow_span(&self) -> &str {
        &self.span[..]
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_kind: TokenKind::Unknown,
            span: String::new(),
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        self.borrow_span()
    }
}
