//! A module defining a node representing an integer.

use std::{fmt, str::FromStr};
use kaleidoscope_lexer::token::{Token, TokenKind};
use crate::prelude::*;

/// The internal representation of an integer,
pub type IntegerType = i128;

/// A node representing an integer. This integer's internal representation
/// depends on [`IntegerType`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerNode {
    value: IntegerType
}

impl IntegerNode {
    /// Create a new [`IntegerNode`] object.
    pub fn new(value: IntegerType) -> Self {
        Self {value}
    }

    /// Get the value of this node as the raw value.
    pub fn get_value(&self) -> IntegerType {
        self.value
    }
}

impl fmt::Display for IntegerNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.value)
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
                format!("Wrong token type passed..."),
                ErrorKind::WrongTokenKind,
                None
            ))
        }
    }
}

impl Node for IntegerNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for IntegerNode {}

impl ExprNode for IntegerNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
