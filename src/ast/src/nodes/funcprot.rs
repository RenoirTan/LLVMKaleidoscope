//! A module for [`FunctionPrototypeNode`].

use std::fmt;
use crate::prelude::*;
use super::IdentifierNode;


#[derive(Debug)]
pub struct FunctionPrototypeNode {
    identifier: Box<IdentifierNode>,
    parameters: Vec<Box<IdentifierNode>>
}

impl fmt::Display for FunctionPrototypeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "def {}{:?}", self.identifier, self.parameters)
    }
}


impl FunctionPrototypeNode {
    /// Create a new [`FunctionPrototypeNode`] object.
    pub fn new(
        identifier: Box<IdentifierNode>,
        parameters: Vec<Box<IdentifierNode>>
    ) -> Self {
        FunctionPrototypeNode {identifier, parameters}
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
    fn node_id(&self) -> NodeId {
        NodeId::new(12)
    }
}
