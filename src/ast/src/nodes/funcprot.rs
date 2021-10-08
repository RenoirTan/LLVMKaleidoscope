//! A module for [`FunctionPrototypeNode`].

use std::fmt;

use inkwell::{module::Linkage, values::AnyValue};
use kaleidoscope_codegen::{error as cgerror, CodeGen, IRRepresentableNode};
use kaleidoscope_macro::iterator_to_str;

use super::IdentifierNode;
use crate::prelude::*;

/// A node representing a function prototype. This contains the name of the
/// function and the parameters the function accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionPrototypeNode {
    identifier: Box<IdentifierNode>,
    parameters: Vec<Box<IdentifierNode>>
}

impl fmt::Display for FunctionPrototypeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = iterator_to_str!(self.parameters.iter(), ", ");
        write!(f, "def {}({})", self.identifier, params)
    }
}


impl FunctionPrototypeNode {
    /// Create a new [`FunctionPrototypeNode`] object.
    pub fn new(identifier: Box<IdentifierNode>, parameters: Vec<Box<IdentifierNode>>) -> Self {
        FunctionPrototypeNode {
            identifier,
            parameters
        }
    }

    /// Get the identifier in the prototype.
    pub fn get_identifier(&self) -> &IdentifierNode {
        &*self.identifier
    }

    /// Get the parameters in the prototype.
    pub fn get_parameters(&self) -> &[Box<IdentifierNode>] {
        &*self.parameters
    }

    /// Get the number of parameters this function should be able to accept.
    pub fn count_parameters(&self) -> usize {
        self.parameters.len()
    }

    /// Get the identifier for the parameter at `index`.
    pub fn nth_parameter(&self, index: usize) -> Option<&IdentifierNode> {
        match self.parameters.get(index) {
            Some(param) => Some(&**param),
            None => None
        }
    }
}

impl Node for FunctionPrototypeNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for FunctionPrototypeNode {}

impl IRRepresentableNode for FunctionPrototypeNode {
    fn represent_node<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> cgerror::Result<Box<dyn AnyValue<'ctx> + 'ctx>> {
        log::trace!("Entering <FunctionPrototypeNode as IRRepresentableNode>::represent_node");
        let name = self.get_identifier().get_identifier();
        log::trace!("Name of function prototype: {}", name);
        let len = self.get_parameters().len();
        log::trace!("Number of parameters: {}", len);
        let num_type = code_gen.get_num_type();
        log::trace!("Generating parameter list");
        let params = {
            let mut p = Vec::with_capacity(len);
            p.resize(len, num_type.into());
            p
        };
        log::trace!("Generating function type");
        let fn_type = num_type.fn_type(&*params, false);
        log::trace!("Registering function ('{}') to module", name);
        let function =
            code_gen
                .get_inner()
                .get_module()
                .add_function(name, fn_type, Some(Linkage::External));
        log::trace!("Function prototype done");
        Ok(Box::new(function))
    }
}
