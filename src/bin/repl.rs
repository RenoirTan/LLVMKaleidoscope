use kaleidoscope_parser::driver::Interpreter;


fn main() {
    println!("Welcome to the Kaleidoscope REPL!");
    let mut repl = Interpreter::default();
	repl.main_loop();
}
