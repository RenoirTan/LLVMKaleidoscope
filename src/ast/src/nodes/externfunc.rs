use std::fmt;
use crate::prelude::*;
use super::FunctionPrototypeNode;


#[derive(Debug)]
pub struct ExternFunctionNode {
    prototype: Box<FunctionPrototypeNode>
}

impl ExternFunctionNode {
    pub fn new(prototype: Box<FunctionPrototypeNode>) -> ExternFunctionNode {
        ExternFunctionNode {prototype}
    }

    pub fn get_prototype(&self) -> &FunctionPrototypeNode {
        &*self.prototype
    }
}

impl Clone for ExternFunctionNode {
    fn clone(&self) -> Self {
        Self::new(self.prototype.clone())
    }
}

impl fmt::Display for ExternFunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "extern {}", self.prototype)
    }
}

impl Node for ExternFunctionNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(16)
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
