use std::iter::Iterator;
use kaleidoscope_lexer::{
    ltuplemut,
    tokenizer::{
        FileStream,
        Tokenizer
    },
};
use crate::{
    error::{Error, ErrorKind, Result},
    parser::Parser
};


const DEFAULT_PROMPT: &'static str = "kaleidoscope::> ";


#[derive(Clone, Debug)]
pub struct Driver {
    interactive: bool,
    prompt: String
}


impl Driver {
    #[inline]
    pub fn new(interactive: bool, prompt: String) -> Self {
        Self {interactive, prompt}
    }

    #[inline]
    pub fn is_interactive(&self) -> bool {
        self.interactive
    }

    #[inline]
    pub fn get_prompt(&self) -> &str {
        &self.prompt[..]
    }

    pub fn handle_function_definition(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> Result<bool> {
        Ok(parser.parse_function(ltuplemut!(istream, tokenizer))?.is_some())
    }

    pub fn handle_extern_function(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> Result<bool> {
        Ok(parser
            .parse_extern_function(ltuplemut!(istream, tokenizer))?
            .is_some())
    }

    pub fn handle_expression(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> Result<bool> {
        Ok(parser
            .parse_top_level_expression(ltuplemut!(istream, tokenizer))?
            .is_some())
    }

    pub fn parse_one(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> Result<bool> {
        if istream.eof_reached() {
            return Ok(false);
        }
        if self.is_interactive() {
            println!("{}", self.prompt);
        }

        let mut gate = 0;

        if self.handle_function_definition(istream, tokenizer, parser)? {
            gate = 1;
        } else if self.handle_extern_function(istream, tokenizer, parser)? {
            gate = 2;
        } else if self.handle_expression(istream, tokenizer, parser)? {
            gate = 3;
        }
        match gate {
            0 => Err(Error::new(
                &"No matching handle found!",
                ErrorKind::ParsingError,
                None
            )),
            1 => {
                println!("Parsed function definition!");
                Ok(true)
            },
            2 => {
                println!("Parsed extern function!");
                Ok(true)
            },
            3 => {
                println!("Parsed expression!");
                Ok(true)
            },
            _ => Err(Error::new(
                &"Driver reached an invalid gate!",
                ErrorKind::ParsingError,
                None
            ))
        }
    }

    pub fn main_loop(
        &self,
        istream: &mut FileStream,
        tokenizer: &mut Tokenizer,
        parser: &mut Parser
    ) -> Result<usize> {
        let mut statements_parsed: usize = 0;
        while self.parse_one(istream, tokenizer, parser)? {
            statements_parsed += 1;
        }
        Ok(statements_parsed)
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new(true, DEFAULT_PROMPT.to_string())
    }
}


pub struct Interpreter<'a> {
    driver: Driver,
    istream: FileStream<'a>,
    tokenizer: Tokenizer,
    parser: Parser,
    proceed_even_if_error: bool,
    can_proceed: bool,
    last_error: Option<Error>
}


impl<'a> Interpreter<'a> {
    pub fn new(interactive: bool, istream: FileStream<'a>) -> Self {
        Self {
            driver: Driver::new(interactive, DEFAULT_PROMPT.to_string()),
            istream,
            tokenizer: Tokenizer::new(),
            parser: Parser::new(),
            proceed_even_if_error: true,
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
        match self.driver.parse_one(
            &mut self.istream,
            &mut self.tokenizer,
            &mut self.parser
        ) {
            Ok(proceed) => {
                self.can_proceed = proceed;
                true
            },
            Err(error) => {
                self.can_proceed = proceed_even_if_error;
                self.last_error = Some(error);
                false
            }
        }
    }

    pub fn main_loop(&mut self) -> usize {
        let mut statements_parsed: usize = 0;
        while {self.parse_once(self.proceed_even_if_error); self.can_proceed} {
            statements_parsed += 1;
        }
        statements_parsed
    }
}


impl<'a> Default for Interpreter<'a> {
    fn default() -> Self {
        Self::new(true, FileStream::default())
    }
}


impl<'a> Iterator for Interpreter<'a> {
    type Item = Result<()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.can_proceed {
            if self.parse_once(self.proceed_even_if_error) {
                Some(Ok(()))
            } else {
                let error = self
                    .take_last_error()
                    .unwrap_or_else(|| Error::new(
                        &"Unknown error",
                        ErrorKind::Other,
                        None
                    ));
                Some(Err(error))
            }
        } else {
            None
        }
    }
}
