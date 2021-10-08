//! A module defining a [`VariableExpressionNode`].

use std::fmt;

use inkwell::values::BasicValueEnum;
use kaleidoscope_codegen::{error as cgerror, CodeGen, IRRepresentableExpression};

use super::IdentifierNode;
use crate::prelude::*;


/// An expression where it's just one variable. This is essentially like
/// `y` in the statement `x = y` in typical "C-like" languages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableExpressionNode {
    identifier: Box<IdentifierNode>
}

impl fmt::Display for VariableExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl VariableExpressionNode {
    /// Create a new [`VariableExpressionNode`] object.
    pub fn new(identifier: Box<IdentifierNode>) -> Self {
        Self { identifier }
    }

    /// Get the identifier.
    pub fn get_identifier(&self) -> &IdentifierNode {
        &*self.identifier
    }
}

impl Node for VariableExpressionNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl IRRepresentableExpression for VariableExpressionNode {
    fn represent_expression<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> cgerror::Result<BasicValueEnum<'ctx>> {
        log::trace!(
            "Entering <VariableExpressionNode as IRRepresentableExpression>::represent_expression"
        );
        let name = self.get_identifier().get_identifier();
        log::trace!("Finding value of '{}'", name);
        match code_gen.get_value(name) {
            Some(value) => {
                log::trace!("Value of '{}' found", name);
                Ok(value)
            },
            None => {
                log::trace!("Could not find identifier named '{}'", name);
                Err(cgerror::Error::new(
                    format!("Could not find identifier named '{}'", name),
                    cgerror::ErrorKind::UndefinedNameError,
                    None
                ))
            }
        }
    }
}

impl NodeType for VariableExpressionNode {}

impl ExprNode for VariableExpressionNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
