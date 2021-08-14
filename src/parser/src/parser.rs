use std::ops;
use kaleidoscope_ast::{
    node::ExprNode,
    nodes::{
        BinaryOperatorNode,
        ExternFunctionNode,
        FloatNode,
        FloatType,
        FunctionNode,
        FunctionPrototypeNode,
        IdentifierNode,
        IntegerNode,
        IntegerType,
        Operator,
        VariableExpressionNode
    }
};
use kaleidoscope_lexer::{
    ltuplemut,
    token::{
        BracketKind,
        FileIndex,
        Keyword,
        Token,
        TokenKind
    },
    tokenizer::LexerTupleMut
};
use kaleidoscope_macro::{
    function_path,
    ok_none,
    return_ok_some
};
use crate::{
    error::{Error, ErrorKind, Result},
    precedence::BinaryOperatorPrecedence
};

pub type ParseResult<T> = Result<Option<Box<T>>>;

#[derive(Clone, Debug, Default)]
struct ParserToken {
    pub token: Option<Token>,
    pub uses: usize
}

#[allow(unused)]
impl ParserToken {
    pub fn new(token: Token) -> Self {
        Self {token: Some(token), uses: 0}
    }

    pub fn unused(&self) -> bool {
        self.uses < 1 && self.token.is_some()
    }

    pub fn set_unused(&mut self) -> &mut Self {
        self.uses = 0;
        self
    }

    pub fn replace(&mut self, token: Token) -> Option<Token> {
        let original = self.token.take();
        self.token = Some(token);
        original
    }

    pub fn replace_used(&mut self, token: Token) -> Option<Token> {
        if self.unused() {
            None
        } else {
            self.replace(token)
        }
    }

    pub fn use_once(&mut self) -> &mut Self {
        self.uses += 1;
        self
    }
}

impl ops::Deref for ParserToken {
    type Target = Option<Token>;
    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

pub struct Parser {
    current_token: ParserToken
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_token: Default::default()
        }
    }

    pub fn next_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token.replace(token);
        Ok(self)
    }

    pub fn replace_used_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token.replace_used(token);
        Ok(self)
    }

    #[inline]
    fn grab_token_from_tokenizer(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> Result<&mut Self> {
        self.next_token(match tokenizer.next_token(stream) {
            Ok(token) => token,
            Err(e) => return Err(Error::from_err(
                Box::new(e),
                ErrorKind::LexerError
            ))
        })
    }

    #[inline]
    fn grab_if_used(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> Result<&mut Self> {
        self.replace_used_token(match tokenizer.next_token(stream) {
            Ok(token) => token,
            Err(e) => return Err(Error::from_err(
                Box::new(e),
                ErrorKind::LexerError
            ))
        })
    }

    #[inline]
    fn get_current_token(&self) -> Option<Token> {
        match *self.current_token {
            Some(ref t) => if t.is_eof() {
                None
            } else {
                Some(t.clone())
            },
            None => None
        }
    }

    fn find_matching_right_round_bracket(
        &mut self,
        lbracket_index: FileIndex,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> Result<Token> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        match self.get_current_token() {
            None => Err(Error::new(
                &format!(
                    "No matching right round bracket found for '(' at {}",
                    lbracket_index
                ),
                ErrorKind::SyntaxError,
                None
            )),
            Some(token) => match token.token_kind {
                TokenKind::Bracket {bracket} => if
                    bracket.side.is_right() &&
                    matches!(bracket.kind, BracketKind::Round)
                {
                    Ok(token)
                } else {
                    Err(Error::new(
                        &format!(
                            "Expected right round bracket at {}",
                            token.start
                        ),
                        ErrorKind::SyntaxError,
                        None
                    ))
                }
                _ => Err(Error::new(
                    &format!(
                        "Expected right round bracket at {}",
                        token.start
                    ),
                    ErrorKind::SyntaxError,
                    None
                ))
            }
        }
    }

    pub fn parse_top_level_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<FunctionNode> {
        println!("[{}] Entering", function_path!()); 
        let expression = match
            self.parse_expression(ltuplemut!(stream, tokenizer))?
        {
            Some(ex) => ex,
            None => return Ok(None)
        };
        let prototype = FunctionPrototypeNode::new(
            Box::new(IdentifierNode::new(String::from(""))),
            Vec::new()
        );
		println!("[{}] Parsed", function_path!());
        Ok(Some(Box::new(FunctionNode::new(
            Box::new(prototype),
            expression
        ))))
    }

    pub fn parse_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        let lhs = self.parse_primary_expression(ltuplemut!(stream, tokenizer))?;
        match lhs {
            None => Ok(None),
            Some(lhs) => self.parse_binary_operator_rhs_expression(
                lhs,
                Operator::Unknown,
                BinaryOperatorPrecedence::Unknown,
                ltuplemut!(stream, tokenizer)
            )
        }
    }

    pub fn parse_primary_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        let integer = self.parse_integer_expression(
            ltuplemut!(stream, tokenizer)
        )?;
        return_ok_some!(integer);
        let float = self.parse_float_expression(
            ltuplemut!(stream, tokenizer)
        )?;
        return_ok_some!(float);
        let rbexpr = self.parse_round_bracket_expression(
            ltuplemut!(stream, tokenizer)
        )?;
        return_ok_some!(rbexpr);
        let variable = self.parse_variable_expression(
            ltuplemut!(stream, tokenizer)
        )?;
        return_ok_some!(variable);
        Ok(None)
    }

    pub fn parse_integer_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.get_current_token());
        if let TokenKind::Integer = token.token_kind {
            let rust_integer = match
                token.borrow_span().parse::<IntegerType>()
            {
                Ok(i) => i,
                Err(e) => return Err(Error::from_err(
                    Box::new(e),
                    ErrorKind::ParsingError
                ))
            };
            self.current_token.use_once();
            Ok(Some(Box::new(IntegerNode::new(rust_integer))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_float_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.get_current_token());
        if let TokenKind::Float = token.token_kind {
            let rust_float = match token.borrow_span().parse::<FloatType>() {
                Ok(f) => f,
                Err(e) => return Err(Error::from_err(
                    Box::new(e),
                    ErrorKind::ParsingError
                ))
            };
            self.current_token.use_once();
            Ok(Some(Box::new(FloatNode::new(rust_float))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_variable_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.get_current_token());
        if let TokenKind::Identifier = token.token_kind {
            self.grab_if_used(ltuplemut!(stream, tokenizer))?;
            let identifier = Box::new(
                IdentifierNode::new(token.borrow_span().to_string())
            );
            Ok(Some(Box::new(VariableExpressionNode::new(identifier))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_binary_operator_rhs_expression(
        &mut self,
        mut lhs: Box<dyn ExprNode>,
        loperator: Operator,
        minimum_operator_precedence: BinaryOperatorPrecedence,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {

        #[inline]
        fn up(
            operator: Operator,
            lhs: Box<dyn ExprNode>,
            rhs: Box<dyn ExprNode>
        ) -> ParseResult<dyn ExprNode> {
            Ok(Some(Box::new(BinaryOperatorNode::new(
                Box::new(operator),
                lhs,
                rhs
            ))))
        }

        // I have no idea what the code below does
        // UPDATE
        // I think I know what this does!
        // So let P be any arbitrary primary expression.
        // Given an expression like this:
        //     P1 + P2 * P3 + P4
        // When you enter this function, you are implicitly given
        // P1 as the left-hand side (abbreviated as LHS). Then you
        // make sure that the next token is an binary operator.
        // If the next token is not an operator, you can return P1
        // by itself.
        //
        // Then the loop looks for the next expression (i.e. P2).
        // If P2 is not found, an error is returned (because
        // Kaleidoscope doesn't have suffix unary operators).
        // However, if P2 is indeed found, we go on to the juicy part
        // of the algorithm:
        //
        // The parser takes a look at the second operator ('*' in this case).
        // Since * has a higher precedence than +, the parser tries to form
        // an expression around P2 and P3 to create (+ P1 (* P2 P3)).
        // As the third operator ('+') does not have a equal or higher
        // priority when compared to *, the parser ends off here.
        //
        // Given the left-hand nature of Kaleidoscope, 2 binary operators
        // with equal precedence (using '+' in this example) will be treated
        // like this (+ (+ P1 P2) P3).
        loop {
            self.grab_if_used(ltuplemut!(stream, tokenizer))?;
            // Unknown as an argument is a special value. It signifies
            let loperator = if let Operator::Unknown = loperator {
                let possible_loperator = match self.get_current_token() {
                    Some(o) => o,
                    None => return Ok(Some(lhs))
                };
                match possible_loperator.token_kind {
                    TokenKind::Operator {operator} => operator,
                    _ => return Ok(Some(lhs))
                }
            } else {
                loperator
            };
            let lprecedence: BinaryOperatorPrecedence = loperator.into();
            if lprecedence < minimum_operator_precedence {
                return Ok(Some(lhs));
            }
            let mut rhs = match self.parse_primary_expression(
                ltuplemut!(stream, tokenizer)
            )? {
                Some(rhs) => rhs,
                None => return Err(Error::new(
                    &format!(
                        "No right-hand side expression after {}",
                        loperator
                    ),
                    ErrorKind::SyntaxError,
                    None
                ))
            };
            self.grab_if_used(ltuplemut!(stream, tokenizer))?;
            let possible_roperator = match self.get_current_token() {
                Some(token) => token,
                None => return up(loperator, lhs, rhs)
            };
            let roperator = match possible_roperator.token_kind {
                TokenKind::Operator {operator} => operator,
                _ => return up(loperator, lhs, rhs)
            };
            let rprecedence =
                BinaryOperatorPrecedence::from_operator(roperator);
            if lprecedence < rprecedence {
                rhs = ok_none!(self.parse_binary_operator_rhs_expression(
                    rhs,
                    roperator,
                    rprecedence,
                    ltuplemut!(stream, tokenizer)
                )?);
            }
            // Collect all expressions to the left-hand side.
            // For a right-hand language, rhs is replaced instead.
            lhs = Box::new(BinaryOperatorNode::new(
                Box::new(loperator),
                lhs,
                rhs
            ));
        }
    }

    pub fn parse_round_bracket_expression(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.get_current_token());
        let left_bracket = match token.token_kind {
            TokenKind::Bracket {bracket} => bracket,
            _ => return Ok(None)
        };
        if !matches!(left_bracket.kind, BracketKind::Round) {
            return Ok(None);
        }
        if !left_bracket.side.is_left() {
            return Err(Error::new(
                &"Mismatched right bracket.",
                ErrorKind::SyntaxError,
                None
            ));
        }
        self.grab_token_from_tokenizer(ltuplemut!(stream, tokenizer))?;
        let expression = match self.parse_expression(
            ltuplemut!(stream, tokenizer)
        )? {
            Some(x) => x,
            None => return Err(Error::new(
                &"Expected expression.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        self.current_token.use_once();
        let token = ok_none!(self.get_current_token());
        let right_bracket = match token.token_kind {
            TokenKind::Bracket {bracket} => bracket,
            _ => return Err(Error::new(
                &"No ending bracket.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        if !left_bracket.cancels_out(right_bracket) {
            return Err(Error::new(
                &"Incompatible brackets",
                ErrorKind::SyntaxError,
                None
            ));
        }
        Ok(Some(expression))
    }

    pub fn parse_function_prototype(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<FunctionPrototypeNode> {
        println!("[{}] Entering", function_path!());
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let def_token = ok_none!(self.get_current_token());
        match def_token.token_kind {
            TokenKind::Keyword {keyword} => match keyword {
                Keyword::Def => (),
                _ => return Ok(None)
            },
            _ => return Ok(None)
        };
		println!("[{}] def keyword found!", function_path!());

        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let function_identifier_token = match self.get_current_token() {
            Some(t) => t,
            None => return Err(Error::new(
                &format!(
                    "Expected function prototype after 'def' at {}",
                    def_token.start
                ),
                ErrorKind::SyntaxError,
                None
            ))
        };
        let function_identifier = match function_identifier_token.token_kind {
            TokenKind::Identifier => Box::new(IdentifierNode::new(
                function_identifier_token.to_string()
            )),
            _ => return Err(Error::new(
                &format!(
                    "Expected valid identifier (name) of function prototype at {}",
                    function_identifier_token.start
                ),
                ErrorKind::SyntaxError,
                None
            ))
        };
		println!(
            "[{}] identifier name: {}",
            function_path!(),
            function_identifier.get_identifier()
        );

        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let lbracket_token = match self.get_current_token() {
            Some(t) => t,
            None => return Err(Error::new(
                &format!(
                    "Expected parameter list after function name at {}",
                    function_identifier_token.start
                ),
                ErrorKind::SyntaxError,
                None
            ))
        };
        match lbracket_token.token_kind {
            TokenKind::Bracket {bracket} if (
                bracket.side.is_left() &&
                matches!(bracket.kind, BracketKind::Round)
            ) => (),
            _ => return Err(Error::new(
                &format!(
                    "Expected '(' to delimit the beginning of the parameter list at {}",
                    function_identifier_token.start
                ),
                ErrorKind::SyntaxError,
                None
            ))
        };
		println!("[{}] left bracket found", function_path!());

        let mut parameters: Vec<Box<IdentifierNode>> = Vec::new();
        let mut ended = false;

        loop {
            self.grab_if_used(ltuplemut!(stream, tokenizer))?;
            if let Some(parameter_token) = self.get_current_token() {
                self.grab_if_used(ltuplemut!(stream, tokenizer))?;
                match self.get_current_token() {
                    Some(comma_token) => match comma_token.token_kind {
                        TokenKind::Comma => (),
                        _ => return Err(Error::new(
                            &format!(
                                "Unexpected token after parameter name at {}",
                                comma_token.start
                            ),
                            ErrorKind::SyntaxError,
                            None
                        ))
                    },
                    None => {ended = true;}
                }
                parameters.push(match parameter_token.token_kind {
                    TokenKind::Identifier => Box::new(IdentifierNode::new(
                        parameter_token.to_string()
                    )),
                    _ => return Err(Error::new(
                        &format!(
                            "Expected identifier at {}",
                            parameter_token.start,
                        ),
                        ErrorKind::SyntaxError,
                        None
                    ))
                });

                if ended {
                    self.find_matching_right_round_bracket(
                        lbracket_token.start,
                        ltuplemut!(stream, tokenizer)
                    )?;
                    break;
                }
            } else {
                return Err(Error::new(
                    &format!(
                        "Unexpected EOF for function prototype at {}",
                        stream.get_index()
                    ),
                    ErrorKind::SyntaxError,
                    None
                ));
            }
        }
		println!("[{}] Parsed", function_path!());

        Ok(Some(Box::new(FunctionPrototypeNode::new(
            function_identifier,
            parameters
        ))))
    }

    pub fn parse_function(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<FunctionNode> {
        let prototype = ok_none!(
            self.parse_function_prototype(ltuplemut!(stream, tokenizer))?
        );
        let body = match self.parse_expression(ltuplemut!(stream, tokenizer))? {
            Some(expression) => expression,
            None => return Err(Error::new(
                &format!(
                    "Expected function body for function prototype at {}",
                    stream.get_index()
                ),
                ErrorKind::SyntaxError,
                None
            ))
        };
        Ok(Some(Box::new(FunctionNode::new(prototype, body))))
    }

    pub fn parse_extern_function(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'_>
    ) -> ParseResult<ExternFunctionNode> {
        println!("[{}] Entering", function_path!());
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let extern_token = ok_none!(self.get_current_token());
        match extern_token.token_kind {
            TokenKind::Keyword {keyword}
                if matches!(keyword, Keyword::Extern) => (),
            _ => {
                if self.current_token.uses > 0 {
                    self.current_token.uses -= 1;
				}
                return Ok(None);
            }
        };
        println!("[{}] extern keyword found", function_path!());

        let prototype = match
            self.parse_function_prototype(ltuplemut!(stream, tokenizer))?
        {
            Some(p) => p,
            None => return Err(Error::new(
                &format!(
                    "No function prototype for extern keyword at {}",
                    extern_token.start
                ),
                ErrorKind::SyntaxError,
                None
            ))
        };
		println!("[{}] Parsed", function_path!());
        Ok(Some(Box::new(ExternFunctionNode::new(prototype))))
    }
}
