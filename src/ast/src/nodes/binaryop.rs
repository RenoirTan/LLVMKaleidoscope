//! A module defining a [`BinaryOperatorNode`].

use std::fmt;

use inkwell::values::{BasicValue, StructValue};
use kaleidoscope_codegen::{error as cgerror, CodeGen, IRRepresentableExpression, builtins::NumValue};

use super::Operator;
use crate::prelude::*;


/// An AST representing an operator with 2 expressions by its side.
/// For example, "1 + 2" is an expression with a binary operator, with
/// '+' being the operator, and '1' and '2' being the 2 arguments of the
/// operator.
#[derive(Debug)]
pub struct BinaryOperatorNode {
    operator: Box<Operator>,
    first:    Box<dyn ExprNode>,
    second:   Box<dyn ExprNode>
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
        Self {
            operator,
            first,
            second
        }
    }

    /// Get the operator in the expression.
    pub fn get_operator(&self) -> &Operator {
        &*self.operator
    }

    /// Get the first argument in the expression.
    pub fn get_first(&self) -> &Box<dyn ExprNode> {
        &self.first
    }

    /// Get the second argument in the expression.
    pub fn get_second(&self) -> &Box<dyn ExprNode> {
        &self.second
    }
}

impl Clone for BinaryOperatorNode {
    fn clone(&self) -> Self {
        Self::new(
            self.operator.clone(),
            self.first.expr_node_clone(),
            self.second.expr_node_clone()
        )
    }
}

impl IRRepresentableExpression for BinaryOperatorNode {
    fn generate_representation<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> cgerror::Result<Box<dyn BasicValue<'ctx> + 'ctx>> {
        let left = NumValue::new(self.first.generate_representation(code_gen)?.as_basic_value_enum().into_struct_value(), code_gen)?;
        let right = NumValue::new(self.second.generate_representation(code_gen)?.as_basic_value_enum().into_struct_value(), code_gen)?;
        let result: StructValue<'ctx> = match *self.operator {
            Operator::Plus => (&left + &right).into(),
            Operator::Minus => (&left - &right).into(),
            Operator::Multiply => (&left - &right).into(),
            Operator::Divide => (&left - &right).into(),
            _ => return Err(cgerror::Error::new(format!("Unknown binary operator: {}", self.operator), cgerror::ErrorKind::UnknownOperationError, None))
        };
        Ok(Box::new(result))
    }
}

impl Node for BinaryOperatorNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for BinaryOperatorNode {}

impl ExprNode for BinaryOperatorNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
