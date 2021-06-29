use std::{
    convert::TryFrom,
    path::PathBuf
};
use clap::{App, Arg};
use kaleidoscope_lexer::tokenizer::{
    FileStream,
    Tokenizer,
    TokenIterator,
    LexerSerializer
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum OutputFormats {
    Debug,
    Json,
    Toml
}

impl OutputFormats {
    pub fn from_string(string: &str) -> Option<Self> {
        Some(match string {
            "debug" => OutputFormats::Debug,
            "json" => OutputFormats::Json,
            "toml" => OutputFormats::Toml,
            _ => return None
        })
    }
}

fn main() {
    let matches = App::new("LLVM Kaleidoscope Tokenizer")
        .version("0.1.0")
        .author("Renoir Tan")
        .about("Tokenizer for LLVM's Kaleidoscope")
        .arg(
            Arg::with_name("input_file")
                .value_name("INPUT_FILE")
                .help(
                    "The input file name. If this is left empty, \
                    standard input is used."
                )
                .multiple(false)
                .required(false)
        )
        /* Commented out because people can just pipe to file
        .arg(
            Arg::with_name("output_file")
                .value_name("OUTPUT_FILE")
                .short("o")
                .long("output-file")
                .help(
                    "The output file name. If this is left empty, \
                    standard output is used."
                )
                .multiple(false)
                .required(false)
        )
        */
        .arg(
            Arg::with_name("output_format")
                .value_name("OUTPUT_FORMAT")
                .short("F")
                .long("output-format")
                .help("The output format. Default is Rust's debug method.")
                .multiple(false)
                .default_value("debug")
        )
        .get_matches();
    let output_format = OutputFormats::from_string(
        matches.value_of("output_format").unwrap()
    ).expect("Invalid output format.");
    let file = match matches.value_of("input_file") {
        Some(path) => FileStream::try_from(&*PathBuf::from(path)).unwrap(),
        None => FileStream::default()
    };
    let tokenizer = Tokenizer::new();
    let mut token_iterator = TokenIterator::new(file, tokenizer);
    match output_format {
        OutputFormats::Debug => {
            for token in &mut token_iterator {
                println!("{:?}", token);
            }
        },
        OutputFormats::Json => {
            let itok = LexerSerializer::new(token_iterator);
            println!("{}", serde_json::to_string_pretty(&itok).unwrap());
        },
        OutputFormats::Toml => {
            let itok = LexerSerializer::new(token_iterator);
            println!("{}", toml::to_string_pretty(&itok).unwrap());
        }
    }
}