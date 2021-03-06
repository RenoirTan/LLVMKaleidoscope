//! A module defining a [`FunctionNode`].

use std::fmt;

use inkwell::values::AnyValueEnum;
use kaleidoscope_codegen::{error as cgerror, CodeGen, IRRepresentableNode};

use super::FunctionPrototypeNode;
use crate::prelude::*;

/// A node representing a function definition.
#[derive(Debug)]
pub struct FunctionNode {
    prototype: Box<FunctionPrototypeNode>,
    body:      Box<dyn ExprNode>
}

impl FunctionNode {
    /// Create a new [`FunctionNode`] object.
    pub fn new(prototype: Box<FunctionPrototypeNode>, body: Box<dyn ExprNode>) -> Self {
        Self { prototype, body }
    }

    /// Get the prototype in the function definition.
    pub fn get_prototype(&self) -> &FunctionPrototypeNode {
        &*self.prototype
    }

    /// Get the body of the function.
    pub fn get_body(&self) -> &dyn ExprNode {
        &*self.body
    }
}

impl Clone for FunctionNode {
    fn clone(&self) -> Self {
        Self::new(self.prototype.clone(), self.body.expr_node_clone())
    }
}

impl fmt::Display for FunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.prototype, self.body)
    }
}

impl Node for FunctionNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for FunctionNode {}


impl IRRepresentableNode for FunctionNode {
    fn represent_node<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> cgerror::Result<AnyValueEnum<'ctx>> {
        log::trace!("Entering <FunctionNode as IRRepresentableNode>::represent_node");
        let name = self.get_prototype().get_identifier().get_value();
        log::trace!("Generating IR for {}'s prototype", name);
        let possible_function = {
            let inner = code_gen.get_inner();
            let module = inner.get_module();
            module.get_function(name)
        };
        let function = match possible_function {
            Some(f) => {
                log::trace!("Pre-declared function prototype found");
                f
            },
            None => {
                log::trace!("Trying to register a new function prototype for '{}'", name);
                let ir = self.get_prototype().represent_node(code_gen)?;
                log::trace!("Function prototype that was created: {:?}", ir);
                ir.into_function_value()
            }
        };
        log::trace!("Creating block for function");
        let block = code_gen.get_context().append_basic_block(function, "entry");
        code_gen.get_inner().get_builder().position_at_end(block);
        log::trace!("Pushing parameter names to named_values table");
        code_gen.clear_named_values();
        for index in 0..self.get_prototype().count_parameters() {
            let param_name = self
                .get_prototype()
                .nth_parameter(index)
                .ok_or_else(|| {
                    unsafe { function.delete() };
                    cgerror::Error::new(
                        format!(
                            "Tried to get parameter at index {} but it does not exist.",
                            index
                        ),
                        cgerror::ErrorKind::Other,
                        None
                    )
                })?
                .get_value()
                .to_string();
            let argument = function.get_nth_param(index as u32).ok_or_else(|| {
                unsafe { function.delete() };
                cgerror::Error::new(
                    format!(
                        "Tried to get argument at index {} but it does not exist.",
                        index
                    ),
                    cgerror::ErrorKind::Other,
                    None
                )
            })?;
            code_gen.set_value(param_name, Box::new(argument));
        }
        log::trace!("Generating return value from expression");
        let retval = self.get_body().represent_expression(code_gen)?;
        log::trace!("Generating return instruction from return value");
        code_gen
            .get_inner()
            .get_builder()
            .build_return(Some(&retval));
        log::trace!("Verifying function...");
        if function.verify(true) {
            log::trace!("'{}' verified", name);
            Ok(AnyValueEnum::FunctionValue(function))
        } else {
            log::trace!(
                "Could not verify '{}', deleting it's declaration from module",
                name
            );
            unsafe { function.delete() };
            Err(cgerror::Error::new(
                format!("Could not verify function '{}'", name),
                cgerror::ErrorKind::CouldNotMakeFunctionError,
                None
            ))
        }
    }
}
