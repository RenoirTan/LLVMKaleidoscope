//! A module defining a [`UnaryOperatorNode`].

use crate::{
    node::{Node, ExprNode},
    NodeId,
};
use super::Operator;

/// An AST representing an operator which takes 1 argument.
/// 
/// # Examples
/// 
/// ```text
/// !true // Boolean Not: false
/// ~50 // Bitwise Not: 205 (8-bit unsigned integer)
/// ```
pub struct UnaryOperatorNode {
    operator: Operator,
    first: Box<dyn ExprNode>
}

impl UnaryOperatorNode {
    /// Get the operator in the expression.
    pub fn get_operator(&self) -> Operator {
        self.operator
    }

    /// Get the first argument in the expression.
    pub fn get_first(&self) -> &dyn ExprNode {
        &*self.first
    }
}

impl Node for UnaryOperatorNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(7)
    }
}

impl ExprNode for UnaryOperatorNode {}
