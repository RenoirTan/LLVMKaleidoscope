use std::{
    convert::AsRef,
    io::{prelude::*, stdin, stdout}
};
use kaleidoscope_parser::driver::Interpreter;


const WELCOME_MESSAGE: &'static str =
r#"Welcome to the Kaleidoscope REPL!
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

    log::info!("STARTING REPL");
    println!("{}", WELCOME_MESSAGE);
    let mut repl = Interpreter::default();
    println!("Statements parsed: {}", repl.main_loop());
    press_enter_to_continue(&"Press enter to continue::> ");
}
