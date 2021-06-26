use std::{
    convert::TryFrom,
    env,
    path::PathBuf
};
use kaleidoscope_lexer::{
    tokenizer::{FileStream, Tokenizer}
};

fn main() {
    let cmd_args = env::args().collect::<Vec<String>>();
    let file = match cmd_args.get(1) {
        Some(path) => {
            let path = PathBuf::from(path);
            FileStream::try_from(&*path).unwrap()
        },
        None => FileStream::default()
    };
    let mut tokenizer = Tokenizer::new(file);
    for token in &mut tokenizer {
        println!("{:?}", token);
    }
}