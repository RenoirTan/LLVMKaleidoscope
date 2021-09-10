//! A module defining a [`BinaryOperatorNode`].

use std::fmt;

use inkwell::values::{BasicValue, BasicValueEnum};
use kaleidoscope_codegen::{error as cgerror, CodeGen, IRRepresentableExpression};

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
        let left = self
            .first
            .generate_representation(code_gen)?
            .as_basic_value_enum();
        let right = self
            .second
            .generate_representation(code_gen)?
            .as_basic_value_enum();
        if let BasicValueEnum::IntValue(linteger) = left {
            let rinteger = if let BasicValueEnum::IntValue(rinteger) = right {
                rinteger
            } else if let BasicValueEnum::FloatValue(rfloat) = right {
                rfloat.const_to_signed_int(code_gen.get_int_type())
            } else {
                return Err(cgerror::Error::new(
                    format!("Bad right type"),
                    cgerror::ErrorKind::TypeError,
                    None
                ));
            };
            Ok(Box::new(match *self.get_operator() {
                Operator::Plus =>
                    code_gen
                        .get_builder()
                        .build_int_add(linteger, rinteger, "add_tmp_int"),
                Operator::Minus =>
                    code_gen
                        .get_builder()
                        .build_int_sub(linteger, rinteger, "sub_tmp_int"),
                Operator::Multiply =>
                    code_gen
                        .get_builder()
                        .build_int_mul(linteger, rinteger, "mul_tmp_int"),
                Operator::Divide =>
                    code_gen
                        .get_builder()
                        .build_int_signed_div(linteger, rinteger, "div_tmp_int"),
                op =>
                    return Err(cgerror::Error::new(
                        format!("Unknown binary operator for integer: {}", op),
                        cgerror::ErrorKind::UnknownOperationError,
                        None
                    )),
            }))
        } else if let BasicValueEnum::FloatValue(lfloat) = left {
            let rfloat = if let BasicValueEnum::IntValue(rinteger) = right {
                rinteger.const_signed_to_float(code_gen.get_float_type())
            } else if let BasicValueEnum::FloatValue(rfloat) = right {
                rfloat
            } else {
                return Err(cgerror::Error::new(
                    format!("Bad right type"),
                    cgerror::ErrorKind::TypeError,
                    None
                ));
            };
            Ok(Box::new(match *self.get_operator() {
                Operator::Plus =>
                    code_gen
                        .get_builder()
                        .build_float_add(lfloat, rfloat, "add_tmp_float"),
                Operator::Minus =>
                    code_gen
                        .get_builder()
                        .build_float_sub(lfloat, rfloat, "sub_tmp_float"),
                Operator::Multiply =>
                    code_gen
                        .get_builder()
                        .build_float_mul(lfloat, rfloat, "mul_tmp_float"),
                Operator::Divide =>
                    code_gen
                        .get_builder()
                        .build_float_div(lfloat, rfloat, "div_tmp_float"),
                op =>
                    return Err(cgerror::Error::new(
                        format!("Unknown binary operator for float: {}", op),
                        cgerror::ErrorKind::UnknownOperationError,
                        None
                    )),
            }))
        } else {
            Err(cgerror::Error::new(
                format!("Bad left type"),
                cgerror::ErrorKind::TypeError,
                None
            ))
        }
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
