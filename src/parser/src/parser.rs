use kaleidoscope_ast::{
    node::{Node, ExprNode},
    nodes::{IntegerType, IntegerNode}
};
use kaleidoscope_lexer::{
    token::{Token, TokenKind, BracketKind, BracketSide},
    tokenizer::Tokenizer
};
use kaleidoscope_macro::ok_none;
use crate::error::{Error, ErrorKind, Result};

pub struct Parser {
    pub current_token: Option<Token>
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_token: None
        }
    }

    pub fn next_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token = Some(token);
        Ok(self)
    }

    #[inline]
    fn grab_token_from_tokenizer(
        &mut self,
        tokenizer: &mut Tokenizer
    ) -> Result<&mut Self> {
        self.next_token(match tokenizer.next_token() {
            Ok(token) => token,
            Err(e) => return Err(Error::from_err(
                Box::new(e),
                ErrorKind::LexerError
            ))
        })
    }

    #[inline]
    fn grab_if_none(&mut self, tokenizer: &mut Tokenizer) -> Result<&mut Self> {
        if self.current_token.is_none() {
            self.grab_token_from_tokenizer(tokenizer)?;
        }
        Ok(self)
    }

    #[inline]
    fn get_current_token(&self) -> Option<Token> {
        match self.current_token {
            Some(ref t) => if t.is_eof() {
                None
            } else {
                Some(t.clone())
            },
            None => None
        }
    }

    pub fn parse_expression(
        &mut self,
        _tokenizer: &mut Tokenizer
    ) -> Result<Option<Box<dyn ExprNode>>> {
        Ok(None)
    }

    pub fn parse_integer(
        &mut self,
        tokenizer: &mut Tokenizer
    ) -> Result<Option<Box<IntegerNode>>> {
        self.grab_if_none(tokenizer)?;
        let token = ok_none!(self.get_current_token());
        if let TokenKind::Integer = token.token_kind {
            let rust_integer = match
                token.borrow_span().parse::<IntegerType>()
            {
                Ok(i) => i,
                Err(e) => return Err(Error::from_err(
                    Box::new(e),
                    ErrorKind::ParsingError
                ))
            };
            self.grab_token_from_tokenizer(tokenizer)?;
            Ok(Some(Box::new(IntegerNode::new(rust_integer))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_round_bracket_expression(
        &mut self,
        tokenizer: &mut Tokenizer
    ) -> Result<Option<Box<dyn ExprNode>>> {
        self.grab_if_none(tokenizer)?;
        let token = ok_none!(self.get_current_token());
        let left_bracket = match token.token_kind {
            TokenKind::Bracket {bracket} => bracket,
            _ => return Ok(None)
        };
        if !matches!(left_bracket.kind, BracketKind::Round) {
            return Ok(None);
        }
        if !left_bracket.side.is_left() {
            return Err(Error::new(
                &"Mismatched right bracket.",
                ErrorKind::SyntaxError,
                None
            ));
        }
        self.grab_token_from_tokenizer(tokenizer)?;
        let expression = match self.parse_expression(tokenizer)? {
            Some(x) => x,
            None => return Err(Error::new(
                &"Expected expression.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        let token = ok_none!(self.get_current_token());
        let right_bracket = match token.token_kind {
            TokenKind::Bracket {bracket} => bracket,
            _ => return Err(Error::new(
                &"No ending bracket.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        if !left_bracket.cancels_out(right_bracket) {
            return Err(Error::new(
                &"Incompatible brackets",
                ErrorKind::SyntaxError,
                None
            ));
        }
        Ok(Some(expression))
    }
}