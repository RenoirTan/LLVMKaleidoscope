use std::{
    convert::TryFrom,
    env,
    io::{Write, stdout},
    path::PathBuf
};
use kaleidoscope_lexer::tokenizer::FileStream;

fn main() {
    let cmd_args = env::args().collect::<Vec<String>>();
    let file = match cmd_args.get(1) {
        Some(path) => {
            let path = PathBuf::from(path);
            FileStream::try_from(&*path).unwrap()
        },
        None => FileStream::default()
    };
    let mut stdout = stdout();
    for unit in file {
        print!("{}", unit);
        stdout.flush().unwrap();
    }
}