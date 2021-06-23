use std::any::Any;
use kaleidoscope_lexer::token::Token;
use crate::error::Result;
use super::NodeId;

pub trait FromToken: Sized {
    fn from_token(token: Token) -> Result<Self>;
}

pub trait Node: Any {
    fn node_id(&self) -> NodeId;
}

pub trait ExprNode: Node {}
