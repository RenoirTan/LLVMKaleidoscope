use std::{
    convert::TryFrom,
    env,
    io::{Write, stdout},
    path::PathBuf
};
use kaleidoscope_lexer::tokenizer::FileStream;

fn main() {
    let cmd_args = env::args().collect::<Vec<String>>();
    let mut file = match cmd_args.get(1) {
        Some(path) => {
            let path = PathBuf::from(path);
            FileStream::try_from(&*path).unwrap()
        },
        None => FileStream::default()
    };
    let mut stdout = stdout();
    for unit in &mut file {
        print!("{:?}\n", unit);
        stdout.flush().unwrap();
    }
    if let Some(error) = file.get_err() {
        println!("[kaleidoscope(bin)::filestream::main:error] {}", error);
    } else {
        println!("[kaleidoscope(bin)::filestream::main] All ok");
    }
}