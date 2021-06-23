use kaleidoscope_lexer::token::{Token, TokenKind};
use crate::{
    node::{FromToken, Node},
    NodeId,
    error::{Error, ErrorKind, Result}
};

pub struct Identifier {
    identifier: String
}

impl Identifier {
    pub fn get_identifier(&self) -> &str {
        &self.identifier[..]
    }
}

impl FromToken for Identifier {
    fn from_token(token: Token) -> Result<Self> {
        if let TokenKind::Identifier = token.token_kind {
            Ok(Self {
                identifier: token.span.clone()
            })
        } else {
            Err(Error::new(
                &format!("Wrong token type passed..."),
                ErrorKind::WrongTokenKind,
                None
            ))
        }
    }
}

impl Node for Identifier {
    fn node_id(&self) -> NodeId {
        NodeId::new(2)
    }
}
