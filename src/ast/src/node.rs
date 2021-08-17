//! Module which defines traits AST nodes can implement.

use std::{any::Any, fmt::{Debug, Display}, ops::Deref};
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
pub trait Node: Any + Debug + Display {
    /// Get the [`NodeId`] of a node. This [`NodeId`] classifies the type
    /// of [`Node`], not the [`Node`] instance itself.
    fn node_id(&self) -> NodeId;

    fn node_clone(&self) -> Box<dyn Node>;
}

/// A node representing an expression.
pub trait ExprNode: Node + Any {
    fn expr_node_clone(&self) -> Box<dyn ExprNode>;
}

impl<T: Node> Node for Box<T> {
    fn node_id(&self) -> NodeId {
        (**self).node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        self.deref().node_clone()
    }
}

impl<T: ExprNode> ExprNode for Box<T> {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        self.deref().expr_node_clone()
    }
}
