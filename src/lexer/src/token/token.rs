//! A token in a file or stream.
//! 
//! See [`Token`] for more comprehensive information.

use serde::{Serialize, Deserialize};
use crate::{
    error::{Error, ErrorKind, Result},
    utils
};
use super::{
    FileIndex,
    TokenKind,
    Keyword,
    Operator,
    Bracket,
    BracketKind
};

/// A token in a Kaleidoscope file.
///
/// This structural representation of a token contains the
/// possible `TokenKind` of the token,
/// the token as a string (stored as `span`),
/// as well as the start and end indices of the token.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub fn is_eof(&self) -> bool {
        matches!(self.token_kind, TokenKind::Eof)
    }

    /// Check if the span in the token is empty or not.
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Add a character into the token.
    /// This method returns a boolean value if no error occurs.
    /// 
    /// If `true` is returned, this means that `unit` is a character that
    /// this token cannot accept, so this token is complete.
    /// If `false` is returned, it means that `unit` has been added to the
    /// token's span and can continue to collect more characters.
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
        }
        self.span.push(unit);
        if utils::is_identifier_start(unit) {
            self.token_kind = TokenKind::Identifier;
        } else if utils::is_decimal_digit(unit) {
            self.token_kind = TokenKind::Integer;
        } else if utils::is_opchar(unit) {
            let operator = Operator::from_string(self.borrow_span());
            self.token_kind = TokenKind::Operator {operator};
        } else if utils::is_bracket(unit) {
            let bracket = Bracket::from_string(self.borrow_span());
            self.token_kind = TokenKind::Bracket {bracket};
        } else if utils::is_comma(unit) {
            self.token_kind = TokenKind::Comma;
        } else if utils::is_dot(unit) {
            self.token_kind = TokenKind::Dot;
        } else if utils::is_semicolon(unit) {
            self.token_kind = TokenKind::Semicolon;
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
            TokenKind::Bracket {..} => self.add_unit_if_bracket(unit, index),
            TokenKind::Comma
            | TokenKind::Dot
            | TokenKind::Semicolon => Ok(true),
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
        if utils::is_identifier(unit) {
            self.span.push(unit);
            Ok(false)
        } else {
            self.resolve(index)
        }
    }

    fn add_unit_if_integer(
        &mut self,
        unit: char,
        _index: FileIndex
    ) -> Result<bool> {
        if utils::is_decimal_digit(unit) {
            self.span.push(unit);
            Ok(false)
        } else if utils::is_fullstop(unit) {
            self.span.push(unit);
            self.token_kind = TokenKind::Float;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn add_unit_if_float(
        &mut self,
        unit: char,
        _index: FileIndex
    ) -> Result<bool> {
        if utils::is_decimal_digit(unit) {
            self.span.push(unit);
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn add_unit_if_operator(
        &mut self,
        unit: char,
        _index: FileIndex
    ) -> Result<bool> {
        if utils::is_opchar(unit) {
            self.span.push(unit);
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn add_unit_if_bracket(
        &mut self,
        _unit: char,
        _index: FileIndex
    ) -> Result<bool> {
        // All brackets are currently only length 1
        Ok(true)
    }

    fn resolve_identifier(&mut self, _index: FileIndex) -> Result<bool> {
        if let Some(keyword) = Keyword::from_string(self.borrow_span()) {
            self.token_kind = TokenKind::Keyword {keyword};
        }
        Ok(true)
    }

    fn resolve_float(&mut self, index: FileIndex) -> Result<bool> {
        match self.span.as_bytes().last() {
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
    }

    fn resolve_operator(&mut self, index: FileIndex) -> Result<bool> {
        match Operator::from_string(self.borrow_span()) {
            Operator::Unknown => Err(Error::new(
                &format!(
                    "Could not guess operator from span '{}' at index {}",
                    self.borrow_span(),
                    index
                ),
                ErrorKind::InvalidCombo,
                None
            )),
            operator => {
                self.token_kind = TokenKind::Operator {operator};
                Ok(true)
            }
        }
    }

    fn resolve_bracket(&mut self, index: FileIndex) -> Result<bool> {
        let bracket = Bracket::from_string(self.borrow_span());
        match bracket.kind {
            BracketKind::Unknown => Err(Error::new(
                &format!(
                    "Invalid bracket from span '{}' at index {}",
                    self.borrow_span(),
                    index
                ),
                ErrorKind::InvalidCombo,
                None
            )),
            _ => {
                self.token_kind = TokenKind::Bracket {bracket};
                Ok(true)
            }
        }
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
            TokenKind::Identifier => self.resolve_identifier(index),
            TokenKind::Float => self.resolve_float(index),
            TokenKind::Operator {..} => self.resolve_operator(index),
            TokenKind::Bracket {..} => self.resolve_bracket(index),
            _ => Ok(true)
        }
    }

    /// Borrow the span as a string slice.
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
