//! A module defining a [`FunctionNode`].

use std::fmt;
use crate::prelude::*;
use super::FunctionPrototypeNode;


#[derive(Debug)]
pub struct FunctionNode {
    prototype: Box<FunctionPrototypeNode>,
    body: Box<dyn ExprNode>
}

impl FunctionNode {
    /// Create a new [`FunctionNode`] object.
    pub fn new(
        prototype: Box<FunctionPrototypeNode>,
        body: Box<dyn ExprNode>
    ) -> Self {
        Self {prototype, body}
    }

    /// Get the prototype in the function definition.
    pub fn get_prototype(&self) -> &FunctionPrototypeNode {
        &*self.prototype
    }

    /// Get the body of the function.
    pub fn get_body(&self) -> &dyn ExprNode {
        &*self.body
    }
}

impl Clone for FunctionNode {
    fn clone(&self) -> Self {
        Self::new(
            self.prototype.clone(),
            self.body.expr_node_clone()
        )
    }
}

impl fmt::Display for FunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.prototype, self.body)
    }
}

impl Node for FunctionNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for FunctionNode {}
