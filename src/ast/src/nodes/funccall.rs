//! A module that defines a node representing a function call.

use std::fmt;

use either::Either;
use inkwell::values::{BasicValue, BasicValueEnum};
use kaleidoscope_codegen::{error as cgerror, CodeGen, IRRepresentableExpression};
use kaleidoscope_macro::iterator_to_str;

use super::IdentifierNode;
use crate::prelude::*;

/// A node representing a function call.
/// This struct stores the name of the function
/// and the list of arguments as a [`Vec`]tor.
#[derive(Debug)]
pub struct FunctionCallNode {
    identifier: Box<IdentifierNode>,
    arguments:  Vec<Box<dyn ExprNode>>
}

impl fmt::Display for FunctionCallNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = iterator_to_str!(self.arguments.iter(), ", ");
        write!(f, "{}({})", self.identifier, args)
    }
}

impl FunctionCallNode {
    /// Create a new instance of a [`FunctionCallNode`].
    pub fn new(identifier: Box<IdentifierNode>, arguments: Vec<Box<dyn ExprNode>>) -> Self {
        Self {
            identifier,
            arguments
        }
    }

    /// Get the name of the function.
    pub fn get_identifier(&self) -> &IdentifierNode {
        &*self.identifier
    }

    /// Get the arguments passed to the function.
    pub fn get_arguments(&self) -> &[Box<dyn ExprNode>] {
        &*self.arguments
    }
}

impl Clone for FunctionCallNode {
    fn clone(&self) -> Self {
        let arguments = self.arguments.iter().map(|a| a.expr_node_clone()).collect();
        Self::new(self.identifier.clone(), arguments)
    }
}

impl IRRepresentableExpression for FunctionCallNode {
    fn represent_expression<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> cgerror::Result<BasicValueEnum<'ctx>> {
        log::trace!(
            "Entering <FunctionCallNode as IRRepresentableExpression>::represent_expression"
        );
        let name = self.get_identifier().get_identifier();
        log::trace!("Trying to find function with the name '{}'", name);
        let function = match code_gen.get_inner().get_module().get_function(name) {
            Some(function) => function,
            None => {
                log::trace!("Could not find function with the name '{}'", name);
                return Err(cgerror::Error::new(
                    format!("Could not find function with name '{}'", name),
                    cgerror::ErrorKind::UndefinedNameError,
                    None
                ));
            }
        };
        log::trace!("Found function with the name '{}", name);
        log::trace!("Pushing arguments to call stack");
        let mut args: Vec<BasicValueEnum> = Vec::with_capacity(self.get_arguments().len());
        for arg in self.get_arguments() {
            args.push(arg.represent_expression(code_gen)?.as_basic_value_enum());
        }
        log::trace!("Generating IR for function call");
        match code_gen
            .get_inner()
            .get_builder()
            .build_call(function, &*args, "call_tmp")
            .try_as_basic_value()
        {
            Either::Left(basic) => {
                log::trace!("Function call successfully generated");
                Ok(basic)
            },
            Either::Right(_instruction) => {
                log::trace!("Invalid function call");
                Err(cgerror::Error::new(
                    format!("Function converted to instruction value"),
                    cgerror::ErrorKind::NotBasicValueError,
                    None
                ))
            }
        }
    }
}

impl Node for FunctionCallNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for FunctionCallNode {}

impl ExprNode for FunctionCallNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
