//! Member which defines structs which represent AST nodes.

pub mod error;
pub mod node;
mod nodeid;
pub mod nodes;

#[cfg(test)]
mod tests;

pub use nodeid::NodeId;

/// Module which re-exports some of the most commonly used items in this
/// member.
pub mod prelude {
    pub use crate::{
        error::{Error, ErrorKind, Result},
        node::{
            reify_expr_node,
            reify_node,
            upcast_expr_node,
            ExprNode,
            FromToken,
            Node,
            NodeType
        },
        nodeid::NodeId
    };
}
