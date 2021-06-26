//! A module defining an operator node.
//! 
//! This node is actually a reexport of the [`Operator`] enum in
//! [`kaleidoscope_lexer::token`] because I was too lazy to reimplement it.

pub use kaleidoscope_lexer::token::Operator;
use crate::{
    node::Node,
    NodeId
};

impl Node for Operator {
    fn node_id(&self) -> NodeId {
        NodeId::new(3)
    }
}
