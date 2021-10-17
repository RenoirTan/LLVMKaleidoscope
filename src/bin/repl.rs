use std::{
    convert::AsRef,
    io::{prelude::*, stdin, stdout}
};

use clap::{App, Arg};
use inkwell::{context::Context, values::AnyValue, OptimizationLevel};
use kaleidoscope_ast::{
    node::{reify_node_ref, NodeEnum},
    nodes::{ExternFunctionNode, FunctionNode}
};
use kaleidoscope_codegen::{create_code_gen, IRRepresentableNode};
use kaleidoscope_optimization::function::optimize_function;
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


fn parse_optimization_level_arg(arg: &str) -> Option<OptimizationLevel> {
    let integer: u32 = arg.parse().ok()?;
    Some(match integer {
        0 => OptimizationLevel::None,
        1 => OptimizationLevel::Less,
        2 => OptimizationLevel::Default,
        3 => OptimizationLevel::Aggressive,
        _ => return None
    })
}


fn main() {
    kaleidoscope_logging::init(None).unwrap();

    let matches = App::new("LLVM Kaleidoscope REPL")
        .version("0.1.0")
        .author("Renoir Tan")
        .about("LLVM Kaleidoscope in a Read-Evaluate-Print-Loop terminal")
        .arg(
            Arg::with_name("optimization_level")
                .value_name("OPT_LEVEL")
                .short("O")
                .long("optimization-level")
                .help("The level of optimisation the REPL should pass functions through.")
                .multiple(false)
                .default_value("0")
        )
        .get_matches();
    let opt_level = matches.value_of("optimization_level").unwrap();
    let opt_level = match parse_optimization_level_arg(opt_level) {
        Some(o) => o,
        None => {
            log::debug!("Invalid OPT_LEVEL: {}", opt_level);
            panic!("Invalid OPT_LEVEL: {}", opt_level);
        }
    };

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
            match node {
                NodeEnum::AnyNode(node) =>
                    if let Some(function) = reify_node_ref::<FunctionNode>(&node) {
                        log::debug!("Function node detected");
                        let ir = function.represent_node(&code_gen).unwrap();
                        let function_value = ir.into_function_value();
                        let code_gen_inner = code_gen.get_inner();
                        let module = code_gen_inner.get_module();
                        let result = optimize_function(&function_value, module, opt_level);
                        if !result {
                            println!(
                                "Failed to optimise function: '{}'",
                                function_value.get_name().to_string_lossy()
                            );
                        }
                        println!("{}", ir.print_to_string().to_string());
                    } else if let Some(external) = reify_node_ref::<ExternFunctionNode>(&node) {
                        log::debug!("Extern function node detected");
                        let ir = external.represent_node(&code_gen).unwrap();
                        println!("{}", ir.print_to_string().to_string());
                    },
                NodeEnum::ExprNode(node) => {
                    log::debug!("Other node type detected");
                    let ir = node.represent_expression(&code_gen).unwrap();
                    println!("{}", ir.print_to_string().to_string());
                }
            }
        }
    }
    press_enter_to_continue(&"Press enter to continue::> ");
}
