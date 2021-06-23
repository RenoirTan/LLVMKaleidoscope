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
