//! A module defining a [`UnaryOperatorNode`].

use std::fmt;
use crate::prelude::*;
use super::Operator;

/// An AST representing an operator which takes 1 argument.
/// 
/// # Examples
/// 
/// ```text
/// !true // Boolean Not: false
/// ~50 // Bitwise Not: 205 (8-bit unsigned integer)
/// ```
#[derive(Debug)]
pub struct UnaryOperatorNode {
    operator: Box<Operator>,
    first: Box<dyn ExprNode>
}

impl UnaryOperatorNode {
    /// Create a new [`UnaryOperatorNode`] object.
    pub fn new(operator: Box<Operator>, first: Box<dyn ExprNode>) -> Self {
        Self {operator, first}
    }

    /// Get the operator in the expression.
    pub fn get_operator(&self) -> &Operator {
        &*self.operator
    }

    /// Get the first argument in the expression.
    pub fn get_first(&self) -> &dyn ExprNode {
        &*self.first
    }
}

impl fmt::Display for UnaryOperatorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.first)
    }
}

impl Clone for UnaryOperatorNode {
    fn clone(&self) -> Self {
        Self::new(
            self.operator.clone(),
            self.first.expr_node_clone()
        )
    }
}

impl Node for UnaryOperatorNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(7)
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl ExprNode for UnaryOperatorNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
