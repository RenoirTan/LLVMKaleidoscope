//! A crate for building LLVM IR using the Inkwell LLVM wrapper.
//! A separate crate is necessary to allow for code reuse.

pub mod codegen;
pub mod int;
pub mod traits;

pub use crate::{codegen::CodeGen, traits::IRRepresentable};
