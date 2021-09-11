//! A module for [`FunctionPrototypeNode`].

use std::fmt;

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
