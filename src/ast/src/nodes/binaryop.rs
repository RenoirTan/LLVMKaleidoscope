//! A module defining a [`BinaryOperatorNode`].

use std::fmt;
use crate::prelude::*;
use super::Operator;

/// An AST representing an operator with 2 expressions by its side.
/// For example, "1 + 2" is an expression with a binary operator, with
/// '+' being the operator, and '1' and '2' being the 2 arguments of the
/// operator.
#[derive(Debug)]
pub struct BinaryOperatorNode {
    operator: Box<Operator>,
    first: Box<dyn ExprNode>,
    second: Box<dyn ExprNode>
}

impl fmt::Display for BinaryOperatorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{}{})", self.operator, self.first, self.second)
    }
}

impl BinaryOperatorNode {
    /// Create a new instance of a [`BinaryOperatorNode`].
    pub fn new(
        operator: Box<Operator>,
        first: Box<dyn ExprNode>,
        second: Box<dyn ExprNode>
    ) -> Self {
        Self {operator, first, second}
    }

    /// Get the operator in the expression.
    pub fn get_operator(&self) -> &Operator {
        &*self.operator
    }

    /// Get the first argument in the expression.
    pub fn get_first(&self) -> &dyn ExprNode {
        &*self.first
    }

    /// Get the second argument in the expression.
    pub fn get_second(&self) -> &dyn ExprNode {
        &*self.second
    }
}

impl Node for BinaryOperatorNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(7)
    }
}

impl ExprNode for BinaryOperatorNode {}
