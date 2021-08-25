use kaleidoscope_parser::driver::Interpreter;


fn main() {
    kaleidoscope_logging::init(None).unwrap();

    log::error!("RED");
    log::warn!("YELLOW");
    log::info!("PLAIN");
    log::debug!("STARTING REPL");
    log::trace!("MINUTE");
    println!("Welcome to the Kaleidoscope REPL!");
    let mut repl = Interpreter::default();
    println!("Statements parsed: {}", repl.main_loop());
}
