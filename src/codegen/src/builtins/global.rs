use inkwell::{
    types::{FunctionType, StructType},
    values::FunctionValue
};

use crate::{
    error::{Error, ErrorKind, Result},
    CodeGen
};

fn func_create_error<'a>(name: &'a str) -> impl Fn() -> Error + 'a {
    move || {
        Error::new(
            format!("Could not create function '{}'", name),
            ErrorKind::CouldNotMakeFunctionError,
            None
        )
    }
}

fn get_binop_types<'ctx>(code_gen: &CodeGen<'ctx>) -> (StructType<'ctx>, FunctionType<'ctx>) {
    let num_type = code_gen.get_num_type();
    let fn_type = num_type.fn_type(&[num_type.into(), num_type.into()], false);
    (num_type, fn_type)
}

pub fn make_num_add<'ctx>(code_gen: &CodeGen<'ctx>) -> Result<FunctionValue<'ctx>> {
    let (_num_type, fn_type) = get_binop_types(code_gen);
    let function = code_gen
        .get_inner()
        .get_module()
        .add_function("__num_add__", fn_type, None);

    let _a = function
        .get_nth_param(0)
        .ok_or_else(func_create_error("__num_add__"))?
        .into_struct_value();

    Ok(function)
}
