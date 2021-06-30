//! A module defining all the nodes in a Kaleidoscope Abstract Syntax Tree.

mod binaryop;
mod funccall;
mod funcprot;
mod function;
mod identifier;
mod integer;
mod operator;
mod unaryop;
mod varexpr;

pub use binaryop::BinaryOperatorNode;
pub use funccall::FunctionCallNode;
pub use funcprot::FunctionPrototypeNode;
pub use function::FunctionNode;
pub use identifier::IdentifierNode;
pub use integer::{IntegerNode, IntegerType};
pub use operator::Operator;
pub use unaryop::UnaryOperatorNode;
pub use varexpr::VariableExpressionNode;
