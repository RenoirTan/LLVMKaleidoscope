//! A module for [`FunctionPrototypeNode`].

use std::fmt;
use crate::prelude::*;
use super::Identifier;


#[derive(Debug)]
pub struct FunctionPrototypeNode {
    identifier: Box<Identifier>,
    parameters: Vec<Box<Identifier>>
}

impl fmt::Display for FunctionPrototypeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "def {}{:?}", self.identifier, self.parameters)
    }
}


impl FunctionPrototypeNode {
    /// Create a new [`FunctionPrototypeNode`] object.
    pub fn new(
        identifier: Box<Identifier>,
        parameters: Vec<Box<Identifier>>
    ) -> Self {
        FunctionPrototypeNode {identifier, parameters}
    }

    /// Get the identifier in the prototype.
    pub fn get_identifier(&self) -> &Identifier {
        &*self.identifier
    }

    /// Get the parameters in the prototype.
    pub fn get_parameters(&self) -> &[Box<Identifier>] {
        &*self.parameters
    }
}

impl Node for FunctionPrototypeNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(12)
    }
}
