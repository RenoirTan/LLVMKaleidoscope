//! A module defining a node that represents an identifier.

use kaleidoscope_lexer::token::{Token, TokenKind};
use crate::{
    node::{FromToken, Node},
    NodeId,
    error::{Error, ErrorKind, Result}
};

/// A struct representing a name or path that can identify an object, function
/// or data structure like classes.
pub struct Identifier {
    identifier: String
}

impl Identifier {
    /// Get the name as a string.
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
