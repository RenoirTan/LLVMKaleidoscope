//! Member which defines structs which represent AST nodes.

pub mod error;
pub mod node;
pub mod nodes;
mod nodeid;

pub use nodeid::NodeId;

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
