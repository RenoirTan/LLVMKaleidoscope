use std::ops;
use kaleidoscope_ast::{
    node::{ExprNode},
    nodes::{
        IntegerNode,
        IntegerType,
        FloatNode,
        FloatType,
        VariableExpressionNode,
        IdentifierNode
    }
};
use kaleidoscope_lexer::{
    ltuplemut,
    token::{Token, TokenKind, BracketKind},
    tokenizer::LexerTupleMut
};
use kaleidoscope_macro::{ok_none, return_ok_some};
use crate::error::{Error, ErrorKind, Result};

pub type ParseResult<T> = Result<Option<Box<T>>>;

#[derive(Clone, Debug, Default)]
struct ParserToken {
    pub token: Option<Token>,
    pub uses: usize
}

#[allow(unused)]
impl ParserToken {
    pub fn new(token: Token) -> Self {
        Self {token: Some(token), uses: 0}
    }

    pub fn unused(&self) -> bool {
        self.uses < 1 && self.token.is_some()
    }

    pub fn set_unused(&mut self) -> &mut Self {
        self.uses = 0;
        self
    }

    pub fn replace(&mut self, token: Token) -> Option<Token> {
        let original = self.token.take();
        self.token = Some(token);
        original
    }

    pub fn replace_used(&mut self, token: Token) -> Option<Token> {
        if self.unused() {
            None
        } else {
            self.replace(token)
        }
    }

    pub fn use_once(&mut self) -> &mut Self {
        self.uses += 1;
        self
    }
}

impl ops::Deref for ParserToken {
    type Target = Option<Token>;
    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

pub struct Parser {
    current_token: ParserToken
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_token: Default::default()
        }
    }

    pub fn next_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token.replace(token);
        Ok(self)
    }

    pub fn replace_used_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token.replace_used(token);
        Ok(self)
    }

    #[inline]
    fn grab_token_from_tokenizer(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> Result<&mut Self> {
        self.next_token(match tokenizer.next_token(stream) {
            Ok(token) => token,
            Err(e) => return Err(Error::from_err(
                Box::new(e),
                ErrorKind::LexerError
            ))
        })
    }

    #[inline]
    fn grab_if_used(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> Result<&mut Self> {
        self.replace_used_token(match tokenizer.next_token(stream) {
            Ok(token) => token,
            Err(e) => return Err(Error::from_err(
                Box::new(e),
                ErrorKind::LexerError
            ))
        })
    }

    #[inline]
    fn get_current_token(&self) -> Option<Token> {
        match *self.current_token {
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
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        return_ok_some!(
            self.parse_integer_expression(ltuplemut!(stream, tokenizer))?
        );
        return_ok_some!(
            self.parse_round_bracket_expression(ltuplemut!(stream, tokenizer))?
        );
        return_ok_some!(
            self.parse_variable_expression(ltuplemut!(stream, tokenizer))?
        );
        Ok(None)
    }

    pub fn parse_integer_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
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
            self.current_token.use_once();
            Ok(Some(Box::new(IntegerNode::new(rust_integer))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_float_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.get_current_token());
        if let TokenKind::Float = token.token_kind {
            let rust_float = match token.borrow_span().parse::<FloatType>() {
                Ok(f) => f,
                Err(e) => return Err(Error::from_err(
                    Box::new(e),
                    ErrorKind::ParsingError
                ))
            };
            self.current_token.use_once();
            Ok(Some(Box::new(FloatNode::new(rust_float))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_variable_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.get_current_token());
        if let TokenKind::Identifier = token.token_kind {
            self.grab_if_used(ltuplemut!(stream, tokenizer))?;
            let identifier = Box::new(
                IdentifierNode::new(token.borrow_span().to_string())
            );
            Ok(Some(Box::new(VariableExpressionNode::new(identifier))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_round_bracket_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
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
        self.grab_token_from_tokenizer(ltuplemut!(stream, tokenizer))?;
        let expression = match self.parse_expression(
            ltuplemut!(stream, tokenizer)
        )? {
            Some(x) => x,
            None => return Err(Error::new(
                &"Expected expression.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        self.current_token.use_once();
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
