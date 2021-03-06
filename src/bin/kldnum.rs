use std::f64::consts::PI;

use inkwell::{context::Context, OptimizationLevel};
use kaleidoscope_codegen::{builtins::number::NumValue, create_code_gen};

fn main() {
    println!("Creating CodeGen");
    let context = Context::create();
    let module = context.create_module("__main__");
    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    let code_gen = create_code_gen(&context, module, engine);
    println!("CodeGen created");

    let num_1 = NumValue::new(code_gen.make_num_from_i128(12345), code_gen.clone()).unwrap();
    println!("Created num_1: 12345");
    println!("{}", num_1);

    let num_2 = NumValue::new(code_gen.make_num_from_f64(PI), code_gen.clone()).unwrap();
    println!("Created num_2: PI");
    println!("{}", num_2);

    let num_3 = &num_1 + &num_2;
    println!("Created num_3: num_1 + num_2");
    println!("{}", num_3);

    println!("num_1.partial_cmp(&num_2): {:?}", num_1.partial_cmp(&num_2));
}
