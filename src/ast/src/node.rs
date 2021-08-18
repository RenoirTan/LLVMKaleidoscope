//! Module which defines traits AST nodes can implement.

use std::{any::Any, fmt::{Debug, Display}};
use kaleidoscope_lexer::token::Token;
use crate::error::Result;
use super::NodeId;

/// A node that implements this trait can convert one token to an instance
/// of itself (e.g. numbers).
pub trait FromToken: Sized {
    /// Create an instance of the node in question from one token.
    fn from_token(token: Token) -> Result<Self>;
}

/// The trait that all node types must implement.
pub trait Node: Any + Debug + Display {
    /// Get the [`NodeId`] of a node. This [`NodeId`] classifies the type
    /// of [`Node`], not the [`Node`] instance itself.
    fn node_id_of_val(&self) -> NodeId;

    fn node_clone(&self) -> Box<dyn Node>;
}

pub trait NodeType {
    /// Get the [`NodeId`] of a node type.
    fn node_id() -> NodeId;
}

/// A node representing an expression.
pub trait ExprNode: Node {
    fn expr_node_clone(&self) -> Box<dyn ExprNode>;
}

/// Convert an [`ExprNode`] to [`Node`].
#[inline]
pub fn upcast_expr_node(node: Box<dyn ExprNode>) -> Box<dyn Node> {
    use crate::nodes::IntegerNode as Arbitrary;
    unsafe {
        // Indirect pointer casting through a concrete pointer
        let pointer = Box::into_raw(node) as *mut Arbitrary as *mut dyn Node;
        Box::from_raw(pointer)
    }
}

/// Convert a node as a trait object and convert it into a concrete node with
/// a known type.
///
/// # Example
///
/// ```
/// use kaleidoscope_ast::{
///     nodes::IntegerNode,
///     node::reify_node   
/// };
/// 
/// let unknown: Box<dyn Node> = Box::new(IntegerNode::new(10));
/// let resolved: Box<IntegerNode> = reify_node(unknown).unwrap();
/// assert_eq!(resolved.get_value(), 10);
/// ```
pub fn reify_node<N>(node: Box<dyn Node>) -> Option<Box<N>>
where
    N: Node + NodeType
{
    if node.node_id_of_val() == N::node_id() { unsafe {
        let pointer = Box::into_raw(node) as *mut N;
        Some(Box::from_raw(pointer))
    }} else {
        None
    }
}

/// Convert an expression node as a trait object and convert it into a node
/// with a known and concrete type.
///
/// # Example
///
/// ```
/// use kaleidoscope_ast::{
///     nodes::IntegerNode,
///     node::reify_expr_node   
/// };
/// 
/// let unknown: Box<dyn ExprNode> = Box::new(IntegerNode::new(10));
/// let resolved: Box<IntegerNode> = reify_expr_node(unknown).unwrap();
/// assert_eq!(resolved.get_value(), 10);
/// ```
pub fn reify_expr_node<N>(node: Box<dyn ExprNode>) -> Option<Box<N>>
where
    N: ExprNode + NodeType
{
    if node.node_id_of_val() == N::node_id() { unsafe {
        let pointer = Box::into_raw(node) as *mut N;
        Some(Box::from_raw(pointer))
    }} else {
        None
    }
}
