//! A module for [`FunctionPrototypeNode`].

use crate::prelude::*;
use super::Identifier;


pub struct FunctionPrototypeNode {
    identifier: Box<Identifier>,
    parameters: Vec<Box<Identifier>>
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
