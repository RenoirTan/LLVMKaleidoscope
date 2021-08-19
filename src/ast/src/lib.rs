//! Member which defines structs which represent AST nodes.

pub mod error;
pub mod node;
pub mod nodes;
mod nodeid;

#[cfg(test)]
mod tests;

pub use nodeid::NodeId;

/// Module which re-exports some of the most commonly used items in this
/// member.
pub mod prelude {
    pub use crate::node::{
        Node,
        NodeType,
        ExprNode,
        FromToken,
        reify_node,
        reify_expr_node,
        upcast_expr_node
    };
    pub use crate::nodeid::NodeId;
    pub use crate::error::{Error, ErrorKind, Result};
}
