use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    types::{FloatType, IntType, StructType},
    values::{BasicValue, FloatValue, IntValue, StructValue}
};

use crate::{
    builtins::number::{make_number_type, NUM_TYPE_NAME},
    error::{Error, ErrorKind, Result},
    int::To64BEWord,
    traits::IRRepresentableExpression
};

/// Create a new LLVM IR generator.
pub fn create_code_gen<'ctx>(
    context: &'ctx Context,
    module: Module<'ctx>,
    engine: ExecutionEngine<'ctx>
) -> CodeGen<'ctx> {
    let mut code_gen = CodeGen::new(context, module, engine);
    code_gen.init();
    code_gen
}

/// A structure representing an LLVM IR generator.
pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module:  Module<'ctx>,
    builder: Builder<'ctx>,
    engine:  ExecutionEngine<'ctx>
}

impl<'ctx: 'val, 'val> CodeGen<'ctx> {
    fn new(context: &'ctx Context, module: Module<'ctx>, engine: ExecutionEngine<'ctx>) -> Self {
        Self {
            context,
            module,
            builder: context.create_builder(),
            engine
        }
    }

    fn init(&mut self) -> &mut Self {
        make_number_type(self);
        self
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

    pub fn get_bool_type(&self) -> IntType<'val> {
        self.get_context().bool_type()
    }

    /// Get the integer type for this context.
    pub fn get_int_type(&self) -> IntType<'val> {
        self.get_context().custom_width_int_type(128)
    }

    /// Get the float type for this context.
    pub fn get_float_type(&self) -> FloatType<'val> {
        self.get_context().f64_type()
    }

    /// Get num type.
    pub fn get_num_type(&self) -> StructType<'val> {
        self.get_module()
            .get_struct_type(NUM_TYPE_NAME)
            .expect(&format!("{} type not initialised yet.", NUM_TYPE_NAME))
    }

    pub fn int_to_float(&self, integer: IntValue<'val>) -> FloatValue<'val> {
        integer.const_signed_to_float(self.get_float_type())
    }

    pub fn float_to_int(&self, float: FloatValue<'val>) -> IntValue<'val> {
        float.const_to_signed_int(self.get_int_type())
    }

    pub fn copy_int(&self, integer: IntValue<'val>) -> IntValue<'val> {
        integer.const_cast(self.get_int_type(), true)
    }

    pub fn copy_float(&self, float: FloatValue<'val>) -> FloatValue<'val> {
        float.const_cast(self.get_float_type())
    }

    pub fn make_num_from_i128(&self, value: i128) -> StructValue<'val> {
        self.make_num_from_int(self.make_i128(value)).unwrap()
    }

    pub fn make_num_from_f64(&self, value: f64) -> StructValue<'val> {
        self.make_num_from_float(self.make_f64(value)).unwrap()
    }

    pub fn make_num_from_int(&self, value: IntValue<'val>) -> Result<StructValue<'val>> {
        let expected_bit_width = self.get_int_type().get_bit_width();
        let gotten_bit_width = value.get_type().get_bit_width();
        if expected_bit_width != gotten_bit_width {
            return Err(Error::new(
                format!(
                    "Expected an integer with a bit width of {}, got one with {} bits instead.",
                    expected_bit_width, gotten_bit_width
                ),
                ErrorKind::TypeError,
                None
            ));
        }
        let integer = value.into();
        let float = self.make_f64(0.0).into();
        let boolean = self.make_bool(true).into();
        let num_type = self.get_num_type();
        Ok(num_type.const_named_struct(&[integer, float, boolean]))
    }

    pub fn make_num_from_float(&self, value: FloatValue<'val>) -> Result<StructValue<'val>> {
        if value.get_type() != self.get_float_type() {
            return Err(Error::new(
                format!("Expected a 64-bit IEEE float."),
                ErrorKind::TypeError,
                None
            ));
        }
        let integer = self.make_i128(0).into();
        let float = value.into();
        let boolean = self.make_bool(false).into();
        let num_type = self.get_num_type();
        Ok(num_type.const_named_struct(&[integer, float, boolean]))
    }

    /// Generate a [`BasicValue`] from an expression that implements
    /// [`IRRepresentableExpression`].
    pub fn make_ir_representable_expression(
        &self,
        node: &dyn IRRepresentableExpression
    ) -> Result<Box<dyn BasicValue<'ctx> + 'ctx>> {
        node.represent_expression(self)
    }

    pub fn make_bool(&self, value: bool) -> IntValue<'val> {
        let bool_type = self.get_bool_type();
        if value {
            bool_type.const_all_ones()
        } else {
            bool_type.const_zero()
        }
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
