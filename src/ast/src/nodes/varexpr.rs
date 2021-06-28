//! A module defining a [`VariableExpressionNode`].

use crate::prelude::*;
use super::Identifier;


/// An expression where it's just one variable. This is essentially like
/// `y` in the statement `x = y` in typical "C-like" languages.
pub struct VariableExpressionNode {
    identifier: Box<Identifier>
}

impl VariableExpressionNode {
    /// Create a new [`VariableExpressionNode`] object.
    pub fn new(identifier: Box<Identifier>) -> Self {
        Self {identifier}
    }

    /// Get the identifier.
    pub fn get_identifier(&self) -> &Identifier {
        &*self.identifier
    }
}

impl Node for VariableExpressionNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(15)
    }
}

impl ExprNode for VariableExpressionNode {}
