use inkwell::values::AnyValue;

use crate::codegen::CodeGen;

/// A node that implements this trait can be converted into LLVM IR.
pub trait IRRepresentable {
    /// Create the LLVM IR for this node.
    fn generate_representation<'ctx>(&self, code_gen: &CodeGen<'ctx>) -> Box<dyn AnyValue<'ctx>>;
}
