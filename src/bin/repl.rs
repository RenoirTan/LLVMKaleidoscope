use kaleidoscope_parser::driver::Interpreter;


const WELCOME_MESSAGE: &'static str =
r#"Welcome to the Kaleidoscope REPL!
Press enter to show the prompt.
To exit, type in a semicolon (';') without an expression before it."#;


fn main() {
    kaleidoscope_logging::init(None).unwrap();

    log::info!("STARTING REPL");
    println!("{}", WELCOME_MESSAGE);
    let mut repl = Interpreter::default();
    println!("Statements parsed: {}", repl.main_loop());
}
