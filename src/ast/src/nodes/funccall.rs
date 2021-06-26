//! An module that defines a node representing a function call.

use crate::{
    node::{Node, ExprNode},
    NodeId
};
use super::Identifier;

/// A node representing a function call.
/// This struct stores the name of the function
/// and the list of arguments as a [`Vec`]tor.
pub struct FunctionCallNode {
    identifier: Identifier,
    arguments: Vec<Box<dyn ExprNode>>
}

impl FunctionCallNode {
    /// Get the name of the function.
    pub fn get_identifier(&self) -> &Identifier {
        &self.identifier
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
