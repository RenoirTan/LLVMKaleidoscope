//! A crate for building LLVM IR using the Inkwell LLVM wrapper.
//! A separate crate is necessary to allow for code reuse.

pub mod builtins;
pub mod codegen;
pub mod error;
pub mod int;
pub mod traits;

#[cfg(test)]
mod tests;

pub use crate::{
    codegen::{create_code_gen, CodeGen},
    traits::{IRRepresentableExpression, IRRepresentableNode}
};
