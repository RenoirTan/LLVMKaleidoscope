//! A module defining a [`FloatNode`].

use std::fmt;

use crate::prelude::*;

/// The type used to represent a Kaleidoscope float. This is equivalent to
/// Rust's `f64` or C's `double`.
pub type FloatType = f64;

/// A node representing a float. The float conforms to IEEE 754's double
/// precision binary float (equivalent to Rust's `f64`).
#[derive(Debug, Clone, PartialEq)]
pub struct FloatNode {
    value: FloatType
}

impl FloatNode {
    /// Create a new [`FloatNode`].
    pub fn new(value: FloatType) -> Self {
        Self { value }
    }

    /// Get the value of the underlying float.
    pub fn get_value(&self) -> FloatType {
        self.value
    }
}

impl Eq for FloatNode {}

impl fmt::Display for FloatNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for FloatNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for FloatNode {}

impl ExprNode for FloatNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
