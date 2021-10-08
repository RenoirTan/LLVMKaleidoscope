//! A module defining an [`ExternFunctionNode`].

use std::fmt;

use inkwell::values::AnyValueEnum;
use kaleidoscope_codegen::{error as cgerror, CodeGen, IRRepresentableNode};

use super::FunctionPrototypeNode;
use crate::prelude::*;

/// An AST representing a function declaration whose definition is defined
/// externally in another library or language.
#[derive(Debug, PartialEq, Eq)]
pub struct ExternFunctionNode {
    prototype: Box<FunctionPrototypeNode>
}

impl ExternFunctionNode {
    /// Create a new [`ExternFunctionNode`].
    pub fn new(prototype: Box<FunctionPrototypeNode>) -> ExternFunctionNode {
        ExternFunctionNode { prototype }
    }

    /// Get the prototype node that contains the name and parameters of the
    /// function.
    pub fn get_prototype(&self) -> &FunctionPrototypeNode {
        &*self.prototype
    }
}

impl Clone for ExternFunctionNode {
    fn clone(&self) -> Self {
        Self::new(self.prototype.clone())
    }
}

impl fmt::Display for ExternFunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "extern {}", self.prototype)
    }
}

impl Node for ExternFunctionNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for ExternFunctionNode {}

impl IRRepresentableNode for ExternFunctionNode {
    fn represent_node<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> cgerror::Result<AnyValueEnum<'ctx>> {
        log::trace!("Entering <ExternFunctionNode as IRRepresentableNode>::represent_node");
        log::trace!("Done");
        self.get_prototype().represent_node(code_gen)
    }
}
