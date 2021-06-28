use kaleidoscope_lexer::token::Token;
use crate::error::Result;

pub struct Parser {
    pub current_token: Option<Token>
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_token: None
        }
    }

    pub fn next_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token = Some(token);
        Ok(self)
    }
}
