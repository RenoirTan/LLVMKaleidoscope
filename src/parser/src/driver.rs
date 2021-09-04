//! The structs used to parse a complete Kaleidoscope programme or read the
//! declarations from an interactive session.

use std::{
    fmt::Display,
    io::{stdout, Write},
    iter::Iterator,
    thread::sleep,
    time::Duration
};

use kaleidoscope_ast::{
    node::{upcast_expr_node, ExprNode, Node},
    nodes::{ExternFunctionNode, FunctionNode}
};
use kaleidoscope_lexer::{
    ltuplemut,
    tokenizer::{FileStream, Tokenizer}
};

use crate::{
    error::{Error, ErrorKind, Result},
    parser::{ParseResult, Parser}
};


/// The default prompt used in an interactive session.
const DEFAULT_PROMPT: &'static str = "kaleidoscope::> ";


pub(crate) fn parser_output_to_str<T>(result: &ParseResult<T>) -> String
where
    T: Display + ?Sized
{
    match result {
        Ok(o) => match o {
            Some(output) => format!("OK {}", output),
            None => format!("NONE")
        },
        Err(e) => format!("{:?}", e)
    }
}


/// The driver that brings input from a file stream to the parser.
#[derive(Clone, Debug)]
pub struct Driver {
    interactive: bool,
    prompt:      String,
    verbosity:   u32
}


impl Driver {
    #[inline]
    pub fn new(interactive: bool, prompt: String, verbosity: u32) -> Self {
        Self {
            interactive,
            prompt,
            verbosity
        }
    }

    #[inline]
    pub fn is_interactive(&self) -> bool {
        self.interactive
    }

    #[inline]
    pub fn get_prompt(&self) -> &str {
        &self.prompt[..]
    }

    #[inline]
    pub fn verbosity(&self) -> u32 {
        self.verbosity
    }

    pub fn handle_function_definition(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> ParseResult<FunctionNode> {
        let result = parser.parse_function(ltuplemut!(istream, tokenizer));
        log::debug!("{:?}", parser_output_to_str(&result));
        result
    }

    pub fn handle_extern_function(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> ParseResult<ExternFunctionNode> {
        let result = parser.parse_extern_function(ltuplemut!(istream, tokenizer));
        log::debug!("{:?}", parser_output_to_str(&result));
        result
    }

    pub fn handle_expression(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> ParseResult<dyn ExprNode> {
        let result = parser.parse_top_level_expression(ltuplemut!(istream, tokenizer));
        log::debug!("{:?}", parser_output_to_str(&result));
        result
    }

    pub fn parse_one(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> ParseResult<dyn Node> {
        if istream.eof_reached() {
            log::debug!("eof reached");
            return Ok(None);
        }
        if let Some(token) = parser.peek_current_token() {
            if token.is_terminating() {
                parser.mark_used();
            }
        }
        if self.is_interactive() {
            print!("{}", self.prompt);
            stdout()
                .flush()
                .map_err(|e| Error::from_err(Box::new(e), ErrorKind::Other))?;
        }

        sleep(Duration::from_millis(1000));

        macro_rules! do_node {
            ($action: expr) => {{
                if self.verbosity() >= 1 {
                    $action;
                }
            }};
        }

        if let Some(node) = self.handle_extern_function(istream, tokenizer, parser)? {
            do_node!(println!("External Function:\n{}", node));
            Ok(Some(node))
        } else if let Some(node) = self.handle_function_definition(istream, tokenizer, parser)? {
            do_node!(println!("Function Definition:\n{}", node.get_prototype()));
            Ok(Some(node))
        } else if let Some(node) = self.handle_expression(istream, tokenizer, parser)? {
            do_node!(println!("Expression:\n{}", node));
            Ok(Some(upcast_expr_node(node)))
        } else {
            do_node!(println!("no handler found"));
            Ok(None)
        }
    }

    pub fn main_loop(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> Result<usize> {
        let mut statements_parsed: usize = 0;
        'main: loop {
            let result = self.parse_one(istream, tokenizer, parser);
            if let Err(error) = result {
                log::error!("{}", error);
                return Err(error);
            } else if let Ok(node) = result {
                if node.is_some() {
                    statements_parsed += 1;
                } else {
                    break 'main;
                }
            }
        }
        Ok(statements_parsed)
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new(true, DEFAULT_PROMPT.to_string(), 1)
    }
}


/// A full interpreter that can parse a programme by itself.
pub struct Interpreter<'a> {
    driver:                Driver,
    istream:               FileStream<'a>,
    tokenizer:             Tokenizer,
    parser:                Parser,
    proceed_even_if_error: bool,
    can_proceed:           bool,
    last_error:            Option<Error>
}


impl<'a> Interpreter<'a> {
    pub fn new(interactive: bool, istream: FileStream<'a>, verbosity: u32) -> Self {
        Self {
            driver: Driver::new(interactive, DEFAULT_PROMPT.to_string(), verbosity),
            istream,
            tokenizer: Tokenizer::new(),
            parser: Parser::new(),
            proceed_even_if_error: false,
            can_proceed: true,
            last_error: None
        }
    }

    pub fn is_done(&self) -> bool {
        self.istream.eof_reached()
    }

    pub fn relinquish_istream(self) -> FileStream<'a> {
        self.istream
    }

    pub fn get_last_error(&self) -> Option<&Error> {
        self.last_error.as_ref()
    }

    pub fn take_last_error(&mut self) -> Option<Error> {
        self.last_error.take()
    }

    pub fn proceed_even_if_error(&mut self) -> &mut Self {
        self.proceed_even_if_error = true;
        self
    }

    pub fn fail_on_error(&mut self) -> &mut Self {
        self.proceed_even_if_error = false;
        self
    }

    pub fn parse_once(&mut self, proceed_even_if_error: bool) -> bool {
        match self
            .driver
            .parse_one(&mut self.istream, &mut self.tokenizer, &mut self.parser)
        {
            Ok(node) => {
                self.can_proceed = node.is_some();
                log::trace!(
                    "expression successfully parsed! continue? {}",
                    self.can_proceed
                );
                true
            },
            Err(error) => {
                log::error!("error: {}", error);
                self.can_proceed = proceed_even_if_error;
                self.last_error = Some(error);
                false
            }
        }
    }

    pub fn main_loop(&mut self) -> usize {
        let mut statements_parsed: usize = 0;
        while {
            self.parse_once(self.proceed_even_if_error);
            self.can_proceed
        } {
            statements_parsed += 1;
        }
        statements_parsed
    }
}


impl<'a> Default for Interpreter<'a> {
    fn default() -> Self {
        Self::new(true, FileStream::default(), 1)
    }
}


impl<'a> Iterator for Interpreter<'a> {
    type Item = Result<()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.can_proceed {
            if self.parse_once(self.proceed_even_if_error) {
                Some(Ok(()))
            } else {
                let error = self.take_last_error().unwrap_or_else(|| {
                    Error::new("Unknown error".to_string(), ErrorKind::Other, None)
                });
                Some(Err(error))
            }
        } else {
            None
        }
    }
}
