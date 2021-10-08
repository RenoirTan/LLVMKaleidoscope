use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::{codegen::CodeGen, error::Result};

pub trait IRRepresentableNode {
    fn represent_node<'ctx>(&self, code_gen: &CodeGen<'ctx>) -> Result<AnyValueEnum<'ctx>>;
}

/// A node that implements this trait can be converted into LLVM IR.
pub trait IRRepresentableExpression {
    /// Create the LLVM IR for this node.
    fn represent_expression<'ctx>(&self, code_gen: &CodeGen<'ctx>) -> Result<BasicValueEnum<'ctx>>;
}
