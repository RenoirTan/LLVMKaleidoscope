use std::{
    convert::AsRef,
    io::{prelude::*, stdin, stdout}
};

use inkwell::{context::Context, OptimizationLevel};
use kaleidoscope_ast::{
    node::reify_node_ref,
    nodes::{ExternFunctionNode, FunctionNode}
};
use kaleidoscope_codegen::{create_code_gen, IRRepresentableNode};
use kaleidoscope_parser::driver::Interpreter;


const WELCOME_MESSAGE: &'static str = r#"Welcome to the Kaleidoscope REPL!
Press enter to show the prompt.
To exit, type in a semicolon (';') without an expression before it."#;


fn press_enter_to_continue(prompt: &dyn AsRef<str>) {
    print!("{}", prompt.as_ref());
    stdout().flush().unwrap();
    let mut _dummy = String::new();
    stdin().read_line(&mut _dummy).unwrap();
}


fn main() {
    kaleidoscope_logging::init(None).unwrap();

    let context = Context::create();
    let module = context.create_module("__main__");
    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    let code_gen = create_code_gen(&context, module, engine);


    log::debug!("STARTING REPL");
    println!("{}", WELCOME_MESSAGE);
    let mut repl = Interpreter::default();
    for node in &mut repl {
        let node = node.unwrap();
        if let Some(node) = node {
            if let Some(function) = reify_node_ref::<FunctionNode>(&node) {
                let ir = function.represent_node(&code_gen).unwrap();
                println!("{}", ir.print_to_string());
            } else if let Some(external) = reify_node_ref::<ExternFunctionNode>(&node) {
                let ir = external.get_prototype().represent_node(&code_gen).unwrap();
                println!("{}", ir.print_to_string());
            } else {
                println!("Something happened <.<");
            }
        }
    }
    press_enter_to_continue(&"Press enter to continue::> ");
}
