// use std::str::FromStr;
// use kaleidoscope_lexer::token::{Token, TokenKind};
use crate::{
    // error::{Error, ErrorKind, Result},
    node::{Node, ExprNode},
    NodeId,
};
use super::Operator;

pub struct BinaryOperatorNode {
    operator: Operator,
    first: Box<dyn ExprNode>,
    second: Box<dyn ExprNode>
}

impl BinaryOperatorNode {
    pub fn get_operator(&self) -> Operator {
        self.operator
    }

    pub fn get_first(&self) -> &dyn ExprNode {
        &*self.first
    }

    pub fn get_second(&self) -> &dyn ExprNode {
        &*self.second
    }
}

impl Node for BinaryOperatorNode {
    fn node_id(&self) -> NodeId {
        NodeId::new(7)
    }
}

impl ExprNode for BinaryOperatorNode {}
