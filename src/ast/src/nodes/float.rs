use std::fmt;
use crate::prelude::*;

pub type FloatType = f64;

#[derive(Debug, Clone)]
pub struct FloatNode {
    value: FloatType
}

impl FloatNode {
    pub fn new(value: FloatType) -> Self {
        Self {value}
    }

    pub fn get_value(&self) -> FloatType {
        self.value
    }
}

impl fmt::Display for FloatNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.value)
    }
}

impl Node for FloatNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(5)
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl ExprNode for FloatNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
