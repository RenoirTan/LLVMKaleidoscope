//! A module that defines a node representing a function call.

use std::fmt;
use kaleidoscope_macro::iterator_to_str;
use crate::prelude::*;
use super::IdentifierNode;

/// A node representing a function call.
/// This struct stores the name of the function
/// and the list of arguments as a [`Vec`]tor.
#[derive(Debug)]
pub struct FunctionCallNode {
    identifier: Box<IdentifierNode>,
    arguments: Vec<Box<dyn ExprNode>>
}

impl fmt::Display for FunctionCallNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = iterator_to_str!(self.arguments.iter(), ", ");
        write!(f, "{}({})", self.identifier, args)
    }
}

impl FunctionCallNode {
    /// Create a new instance of a [`FunctionCallNode`].
    pub fn new(
        identifier: Box<IdentifierNode>,
        arguments: Vec<Box<dyn ExprNode>>
    ) -> Self {
        Self {identifier, arguments}
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

impl Node for FunctionCallNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(11)
    }
}
