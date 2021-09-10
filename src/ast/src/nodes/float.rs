use std::fmt;

use inkwell::values::BasicValue;
use kaleidoscope_codegen::{error::Result as CodegenResult, CodeGen, IRRepresentable};

use crate::prelude::*;

pub type FloatType = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct FloatNode {
    value: FloatType
}

impl FloatNode {
    pub fn new(value: FloatType) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> FloatType {
        self.value
    }
}

impl Eq for FloatNode {}

impl fmt::Display for FloatNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.value)
    }
}

impl IRRepresentable for FloatNode {
    fn generate_representation<'ctx>(
        &self,
        code_gen: &CodeGen<'ctx>
    ) -> CodegenResult<Box<dyn BasicValue<'ctx> + 'ctx>> {
        Ok(Box::new(code_gen.make_f64(self.get_value())))
    }
}

impl Node for FloatNode {
    fn node_id_of_val(&self) -> NodeId {
        Self::node_id()
    }

    fn node_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl NodeType for FloatNode {}

impl ExprNode for FloatNode {
    fn expr_node_clone(&self) -> Box<dyn ExprNode> {
        Box::new(self.clone())
    }
}
