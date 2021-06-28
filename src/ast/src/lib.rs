//! Member which defines structs which represent AST nodes.

pub mod error;
pub mod node;
pub mod nodes;
mod nodeid;

pub use nodeid::NodeId;

pub mod prelude {
    pub use crate::node::{Node, ExprNode, FromToken};
    pub use crate::nodeid::NodeId;
    pub use crate::error::{Error, ErrorKind, Result};
}
