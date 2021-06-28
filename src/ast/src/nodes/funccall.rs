//! A module that defines a node representing a function call.

use crate::prelude::*;
use super::Identifier;

/// A node representing a function call.
/// This struct stores the name of the function
/// and the list of arguments as a [`Vec`]tor.
pub struct FunctionCallNode {
    identifier: Box<Identifier>,
    arguments: Vec<Box<dyn ExprNode>>
}

impl FunctionCallNode {
    /// Create a new instance of a [`FunctionCallNode`].
    pub fn new(identifier: Box<Identifier>, arguments: Vec<Box<dyn ExprNode>>) -> Self {
        Self {identifier, arguments}
    }

    /// Get the name of the function.
    pub fn get_identifier(&self) -> &Identifier {
        &*self.identifier
    }

    /// Get the arguments passed to the function.
    pub fn get_arguments(&self) -> &[Box<dyn ExprNode>] {
        &*self.arguments
    }
}

impl Node for FunctionCallNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(11)
    }
}
