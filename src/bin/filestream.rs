use std::{
    convert::TryFrom,
    env,
    io::{stdout, Write},
    path::PathBuf
};

use kaleidoscope_lexer::tokenizer::FileStream;

fn main() {
    kaleidoscope_logging::init(None).unwrap();

    let cmd_args = env::args().collect::<Vec<String>>();
    let mut file = match cmd_args.get(1) {
        Some(path) => {
            let path = PathBuf::from(path);
            FileStream::try_from(&*path).unwrap()
        },
        None => FileStream::default()
    };
    let mut stdout = stdout();
    while let Some(unit) = file.next() {
        print!("{:?} {}\n", unit, file.get_index());
        stdout.flush().unwrap();
    }
    if let Some(error) = file.get_err() {
        println!("[kaleidoscope(bin)::filestream::main:error] {}", error);
    } else {
        println!("[kaleidoscope(bin)::filestream::main] All ok");
    }
}
