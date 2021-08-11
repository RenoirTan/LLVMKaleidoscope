//! A module defining a [`VariableExpressionNode`].

use std::fmt;
use crate::prelude::*;
use super::IdentifierNode;


/// An expression where it's just one variable. This is essentially like
/// `y` in the statement `x = y` in typical "C-like" languages.
#[derive(Debug, Clone)]
pub struct VariableExpressionNode {
    identifier: Box<IdentifierNode>
}

impl fmt::Display for VariableExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.identifier)
    }
}

impl VariableExpressionNode {
    /// Create a new [`VariableExpressionNode`] object.
    pub fn new(identifier: Box<IdentifierNode>) -> Self {
        Self {identifier}
    }

    /// Get the identifier.
    pub fn get_identifier(&self) -> &IdentifierNode {
        &*self.identifier
    }
}

impl Node for VariableExpressionNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(15)
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl ExprNode for VariableExpressionNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
