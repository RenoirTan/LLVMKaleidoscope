//! Module which defines traits AST nodes can implement as well as utility
//! functions that act on these nodes.

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
/// 
/// DO NOT USE FOR NOW.
#[inline]
#[warn(unstable_features)]
pub fn upcast_expr_node(node: Box<dyn ExprNode>) -> Box<dyn Node> {
    use crate::nodes::IntegerNode as Arbitrary;
    unsafe {
        // Indirect pointer casting through a concrete pointer
        // Unfortunately this workaround causes the final vtable to point to
        // IntegerNode's methods.
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
///     node::{Node, reify_node}
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
///     node::{ExprNode, reify_expr_node}
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

/// Convert a boxed [`Node`] to an immutable reference to a node with a
/// concrete type.
///
/// # Example
///
/// ```
/// use kaleidoscope_ast::{
///     nodes::IntegerNode,
///     node::{Node, reify_node_ref}
/// };
/// 
/// let unknown: Box<dyn Node> = Box::new(IntegerNode::new(34));
/// let resolved: &IntegerNode = reify_node_ref(&unknown).unwrap();
/// assert_eq!(resolved.get_value(), 34);
/// ```
pub fn reify_node_ref<N>(node: &Box<dyn Node>) -> Option<&N>
where
    N: Node + NodeType
{
    if node.node_id_of_val() == N::node_id() { unsafe {
        let reference = &*(&**node as *const dyn Node as *const N);
        Some(reference)
    }} else {
        None
    }
}

/// Convert a boxed [`ExprNode`] to an immutable reference to a node with a
/// concrete type.
///
/// # Example
///
/// ```
/// use kaleidoscope_ast::{
///     nodes::IntegerNode,
///     node::{ExprNode, reify_expr_node_ref}
/// };
/// 
/// let unknown: Box<dyn ExprNode> = Box::new(IntegerNode::new(34));
/// let resolved: &IntegerNode = reify_expr_node_ref(&unknown).unwrap();
/// assert_eq!(resolved.get_value(), 34);
/// ```
pub fn reify_expr_node_ref<N>(node: &Box<dyn ExprNode>) -> Option<&N>
where
    N: ExprNode + NodeType
{
    if node.node_id_of_val() == N::node_id() { unsafe {
        let reference = &*(&**node as *const dyn ExprNode as *const N);
        Some(reference)
    }} else {
        None
    }
}

/// Convert a boxed [`Node`] to a mutable reference to a node with a concrete
/// type.
pub fn reify_node_mut<N>(node: &mut Box<dyn Node>) -> Option<&mut N>
where
    N: Node + NodeType
{
    if node.node_id_of_val() == N::node_id() { unsafe {
        let reference = &mut *(&mut **node as *mut dyn Node as *mut N);
        Some(reference)
    }} else {
        None
    }
}

/// Convert a boxed [`ExprNode`] to a mutable reference to a node with a
/// concrete type.
pub fn reify_expr_node_mut<N>(node: &mut Box<dyn ExprNode>) -> Option<&mut N>
where
    N: ExprNode + NodeType
{
    if node.node_id_of_val() == N::node_id() { unsafe {
        let reference = &mut *(&mut **node as *mut dyn ExprNode as *mut N);
        Some(reference)
    }} else {
        None
    }
}
