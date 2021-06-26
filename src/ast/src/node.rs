//! Module which defines traits AST nodes can implement.

use std::any::Any;
use kaleidoscope_lexer::token::Token;
use crate::error::Result;
use super::NodeId;

/// A node that implements this trait can convert one token to an instance
/// of itself (e.g. numbers).
pub trait FromToken: Sized {
    /// Create an instance of the node in question from one token.
    fn from_token(token: Token) -> Result<Self>;
}

/// The trait that all node types must implement.
pub trait Node: Any {
    /// Get the [`NodeId`] of a node. This [`NodeId`] classifies the type
    /// of [`Node`], not the [`Node`] instance itself.
    fn node_id(&self) -> NodeId;
}

/// A node representing an expression.
pub trait ExprNode: Node {}
