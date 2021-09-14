use inkwell::{
    types::StructType,
    values::{AggregateValue, BasicValueEnum, FloatValue, IntValue, StructValue}
};

use crate::{
    error::{Error, ErrorKind, Result},
    CodeGen
};

pub const NUM_TYPE_NAME: &'static str = "num";

pub fn make_number_type<'ctx>(code_gen: &CodeGen<'ctx>) -> StructType<'ctx> {
    let int_type = code_gen.get_int_type().into();
    let float_type = code_gen.get_float_type().into();
    let bool_type = code_gen.get_context().bool_type().into();
    let struct_type = code_gen.get_context().opaque_struct_type(NUM_TYPE_NAME);
    struct_type.set_body(&[int_type, float_type, bool_type], true);
    code_gen
        .get_module()
        .get_struct_type(NUM_TYPE_NAME)
        .expect(&format!("{} could not be created", NUM_TYPE_NAME))
}


pub struct NumValue<'ctx: 'cdg, 'cdg> {
    value:    StructValue<'ctx>,
    code_gen: &'cdg CodeGen<'ctx>
}


impl<'ctx: 'cdg, 'cdg> NumValue<'ctx, 'cdg> {
    pub fn new(value: StructValue<'ctx>, code_gen: &'cdg CodeGen<'ctx>) -> Result<Self> {
        if value.get_type() != code_gen.get_num_type() {
            Err(Error::new(
                format!("Invalid type for NumValue"),
                ErrorKind::TypeError,
                None
            ))
        } else {
            Ok(Self { value, code_gen })
        }
    }

    pub fn make_i128(value: i128, code_gen: &'cdg CodeGen<'ctx>) -> Self {
        Self {
            value: code_gen.make_num_from_i128(value),
            code_gen
        }
    }

    pub fn make_f64(value: f64, code_gen: &'cdg CodeGen<'ctx>) -> Self {
        Self {
            value: code_gen.make_num_from_f64(value),
            code_gen
        }
    }

    pub fn destructure(&self) -> [BasicValueEnum; 3] {
        [
            self.value.const_extract_value(&mut [0]),
            self.value.const_extract_value(&mut [1]),
            self.value.const_extract_value(&mut [2])
        ]
    }

    pub fn get_int_switch(&self) -> IntValue<'ctx> {
        self.value.const_extract_value(&mut [2]).into_int_value()
    }

    pub fn is_int(&self) -> bool {
        self.get_int_switch() == self.code_gen.make_bool(true)
    }

    pub fn is_float(&self) -> bool {
        !self.is_int()
    }

    pub fn get_raw_int_value(&self) -> IntValue<'ctx> {
        self.value.const_extract_value(&mut [0]).into_int_value()
    }

    pub fn get_raw_float_value(&self) -> FloatValue<'ctx> {
        self.value.const_extract_value(&mut [1]).into_float_value()
    }
}
