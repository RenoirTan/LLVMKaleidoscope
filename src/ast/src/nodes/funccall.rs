//! A module that defines a node representing a function call.

use std::fmt;

use kaleidoscope_macro::iterator_to_str;

use super::IdentifierNode;
use crate::prelude::*;

/// A node representing a function call.
/// This struct stores the name of the function
/// and the list of arguments as a [`Vec`]tor.
#[derive(Debug)]
pub struct FunctionCallNode {
    identifier: Box<IdentifierNode>,
    arguments:  Vec<Box<dyn ExprNode>>
}

impl fmt::Display for FunctionCallNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = iterator_to_str!(self.arguments.iter(), ", ");
        write!(f, "{}({})", self.identifier, args)
    }
}

impl FunctionCallNode {
    /// Create a new instance of a [`FunctionCallNode`].
    pub fn new(identifier: Box<IdentifierNode>, arguments: Vec<Box<dyn ExprNode>>) -> Self {
        Self {
            identifier,
            arguments
        }
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

impl Clone for FunctionCallNode {
    fn clone(&self) -> Self {
        let arguments = self.arguments.iter().map(|a| a.expr_node_clone()).collect();
        Self::new(self.identifier.clone(), arguments)
    }
}

impl Node for FunctionCallNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for FunctionCallNode {}

impl ExprNode for FunctionCallNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
