use std::str::FromStr;
use kaleidoscope_lexer::token::{Token, TokenKind};
use crate::{
    error::{Error, ErrorKind, Result},
    node::{FromToken, Node, ExprNode},
    NodeId
};

pub type IntegerType = i128;

pub struct IntegerNode {
    value: IntegerType
}

impl IntegerNode {
    pub fn get_value(&self) -> IntegerType {
        self.value
    }
}

impl FromToken for IntegerNode {
    fn from_token(token: Token) -> Result<Self> {
        if let TokenKind::Integer = token.token_kind {
            let span = token.borrow_span();
            let value: IntegerType = match FromStr::from_str(span) {
                Ok(v) => v,
                Err(e) => return Err(Error::from_err(
                    Box::new(e),
                    ErrorKind::TypeCasting
                ))
            };
            Ok(Self {value})
        } else {
            Err(Error::new(
                &format!("Wrong token type passed..."),
                ErrorKind::WrongTokenKind,
                None
            ))
        }
    }
}

impl Node for IntegerNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(4)
    }
}

impl ExprNode for IntegerNode {}
