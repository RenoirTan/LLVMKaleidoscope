use std::fmt;
use crate::prelude::*;

pub type FloatType = f64;

#[derive(Debug)]
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
}

impl ExprNode for FloatNode {}
