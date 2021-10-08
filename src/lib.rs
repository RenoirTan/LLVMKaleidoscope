//! This is the root page for my implementation of a parser and compiler for
//! LLVM's Kaleidoscope language
//! (<https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html>).
//! I did started following this tutorial out of boredom and am now regretting
//! this decision for not doing it in C++ like LLVM did in their
//! documentation.
//!
//! # Purpose of the main library
//!
//! The main library is mainly used to redirect readers of this piece of
//! documentation to the correct members in the cargo workspace and does not
//! contain any special functions or types. If I do ever define anything here,
//! it's usually a re-export or wrapper of one of the functions/types in one
//! of the other member libraries.
//!
//! # Catalogue
//!
//! In alphabetical order:
//!
//! 1. [`kaleidoscope_ast`] - Defines types that can represent the grammar of
//! Kaleidoscope as an Abstract Syntax Tree.
//! 2. [`kaleidoscope_codegen`] - Library for generating LLVM IR for
//! Kaleidoscope.
//! 3. [`kaleidoscope_error`] - Defines a custom generic error type which can
//! be used to fit the purposes of each member library.
//! 4. [`kaleidoscope_lexer`] - Library which can parses file/stream input
//! into tokens.
//! 5. [`kaleidoscope_logging`] - Library which sets up the logging facilities
//! used by the binaries in this crate.
//! 6. [`kaleidoscope_macro`] - Assortment of macros which I use frequently in
//! this crate.
//! 7. [`kaleidoscope_parser`] - The parser. This library provides definitions
//! to turn a series of tokens into an abstract syntax tree.

pub use kaleidoscope_ast::{node::Node, nodes, NodeId};
pub use kaleidoscope_codegen::{
    int::To64LLVMWord,
    CodeGen,
    IRRepresentableExpression,
    IRRepresentableNode
};
pub use kaleidoscope_error::{Error, ErrorKind, Result};
pub use kaleidoscope_lexer::{
    token::{Token, TokenKind},
    tokenizer::Tokenizer
};
pub use kaleidoscope_macro::{
    function_name,
    function_path,
    hash_map,
    impl_display,
    untrimmed_function_path
};
pub use kaleidoscope_parser::{
    driver::{Driver, Interpreter},
    parser::{ParseResult, Parser}
};
