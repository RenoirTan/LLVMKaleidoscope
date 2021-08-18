//! A module defining an operator node.
//! 
//! This node is actually a reexport of the [`Operator`] enum in
//! [`kaleidoscope_lexer::token`] because I was too lazy to reimplement it.

pub use kaleidoscope_lexer::token::Operator;
use crate::prelude::*;

impl Node for Operator {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for Operator {
    fn node_id() -> NodeId {
        NodeId::new(3)
    }
}
