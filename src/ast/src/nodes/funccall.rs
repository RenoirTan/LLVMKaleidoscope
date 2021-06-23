use crate::{
    node::{Node, ExprNode},
    NodeId
};
use super::Identifier;

pub struct FunctionCallNode {
    identifier: Identifier,
    arguments: Vec<Box<dyn ExprNode>>
}

impl FunctionCallNode {
    pub fn get_identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn get_arguments(&self) -> &[Box<dyn ExprNode>] {
        &*self.arguments
    }
}

impl Node for FunctionCallNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(11)
    }
}
