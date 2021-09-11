use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    types::{FloatType, IntType},
    values::{BasicValue, FloatValue, IntValue}
};

use crate::{error::Result, int::To64BEWord, traits::IRRepresentableExpression};

/// A structure representing an LLVM IR generator.
pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module:  Module<'ctx>,
    builder: Builder<'ctx>,
    engine:  ExecutionEngine<'ctx>
}

impl<'ctx: 'val, 'val> CodeGen<'ctx> {
    /// Create a new LLVM IR generator.
    pub fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        engine: ExecutionEngine<'ctx>
    ) -> Self {
        Self {
            context,
            module,
            builder: context.create_builder(),
            engine
        }
    }

    /// Get the context.
    pub fn get_context(&self) -> &'ctx Context {
        &self.context
    }

    /// Get the module.
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Get the builder.
    pub fn get_builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    /// Get the execution engine.
    pub fn get_engine(&self) -> &ExecutionEngine<'ctx> {
        &self.engine
    }

    /// Get the integer type for this context.
    pub fn get_int_type(&self) -> IntType<'val> {
        self.get_context().custom_width_int_type(128)
    }

    /// Get the float type for this context.
    pub fn get_float_type(&self) -> FloatType<'val> {
        self.get_context().f64_type()
    }

    /// Generate a [`BasicValue`] from an expression that implements
    /// [`IRRepresentableExpression`].
    pub fn make_ir_representable_expression(
        &self,
        node: &dyn IRRepresentableExpression
    ) -> Result<Box<dyn BasicValue<'ctx> + 'ctx>> {
        node.generate_representation(self)
    }

    /// Create a u8 value from this context.
    pub fn make_u8(&self, value: u8) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_u8(value))
    }

    /// Create a u16 value from this context.
    pub fn make_u16(&self, value: u16) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_u16(value))
    }

    /// Create a u32 from this context.
    pub fn make_u32(&self, value: u32) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_u32(value))
    }

    /// Create a u64 from this context.
    pub fn make_u64(&self, value: u64) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_u64(value))
    }

    /// Create a u128 from this context.
    pub fn make_u128(&self, value: u128) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_u128(value))
    }

    /// Create an i8 from this context.
    pub fn make_i8(&self, value: i8) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_i8(value))
    }

    /// Create an i16 from this context.
    pub fn make_i16(&self, value: i16) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_i16(value))
    }

    /// Create an i32 from this context.
    pub fn make_i32(&self, value: i32) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_i32(value))
    }

    /// Create an i64 from this context.
    pub fn make_i64(&self, value: i64) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_i64(value))
    }

    /// Create an i128 from this context.
    pub fn make_i128(&self, value: i128) -> IntValue<'val> {
        self.get_int_type()
            .const_int_arbitrary_precision(&To64BEWord::from_i128(value))
    }

    /// Create a f32 from this context.
    pub fn make_f32(&self, value: f32) -> FloatValue<'val> {
        self.get_float_type().const_float(value as f64)
    }

    /// Create a f64 from this context.
    pub fn make_f64(&self, value: f64) -> FloatValue<'val> {
        self.get_float_type().const_float(value)
    }
}
