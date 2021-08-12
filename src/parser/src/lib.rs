//! The parser which uses the output from the tokenizers in
//! [`kaleidoscope_lexer`] to build an abstract syntax tree with nodes defined
//! in [`kaleidoscope_ast`].

pub mod driver;
pub mod error;
pub mod parser;
pub mod precedence;
