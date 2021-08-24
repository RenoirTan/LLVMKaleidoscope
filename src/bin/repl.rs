use kaleidoscope_parser::driver::Interpreter;


fn main() {
    kaleidoscope_logging::init(None).unwrap();

    log::info!("STARTING REPL");
    println!("Welcome to the Kaleidoscope REPL!");
    let mut repl = Interpreter::default();
    println!("Statements parsed: {}", repl.main_loop());
}
