use inkwell::values::BasicValue;

use crate::{codegen::CodeGen, error::Result};

/// A node that implements this trait can be converted into LLVM IR.
pub trait IRRepresentableExpression {
    /// Create the LLVM IR for this node.
    fn generate_representation<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> Result<Box<dyn BasicValue<'ctx> + 'ctx>>;
}
