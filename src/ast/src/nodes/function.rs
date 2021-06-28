//! A module defining a [`FunctionNode`].

use crate::prelude::*;
use super::FunctionPrototypeNode;


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
