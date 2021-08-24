use kaleidoscope_parser::driver::Interpreter;


fn main() {
    println!("Welcome to the Kaleidoscope REPL!");
    let mut repl = Interpreter::default();
    println!("Statements parsed: {}", repl.main_loop());
}
