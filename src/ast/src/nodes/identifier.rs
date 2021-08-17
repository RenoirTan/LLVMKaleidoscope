//! A module defining a node that represents an identifier.

use std::fmt;
use kaleidoscope_lexer::token::{Token, TokenKind};
use crate::prelude::*;

/// A struct representing a name or path that can identify an object, function
/// or data structure like classes.
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    identifier: String
}

impl IdentifierNode {
    /// Create a new [`IdentifierNode`] object.
    pub fn new(identifier: String) -> Self {
        IdentifierNode {identifier}
    }

    /// Get the name as a string.
    pub fn get_identifier(&self) -> &str {
        &self.identifier[..]
    }
}

impl fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl FromToken for IdentifierNode {
    fn from_token(token: Token) -> Result<Self> {
        if let TokenKind::Identifier = token.token_kind {
            Ok(IdentifierNode {
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

impl Node for IdentifierNode {
    fn node_id_of_val(&self) -> NodeId {
        NodeId::new(2)
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
