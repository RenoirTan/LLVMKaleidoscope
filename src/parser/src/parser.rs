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
        LEFT_ROUND_BRACKET,
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

    #[inline]
    pub fn unused(&self) -> bool {
        self.uses < 1 && self.token.is_some()
    }

    #[inline]
    pub fn set_unused(&mut self) -> &mut Self {
        self.uses = 0;
        self
    }

    pub fn replace(&mut self, token: Token) -> Option<Token> {
        // println!("[{}] new token: {:?}\n", function_path!(), token);
        let original = self.token.take();
        self.token = Some(token);
        self.uses = 0; // Do not remove this line
        original
    }

    #[inline]
    pub fn replace_used(&mut self, token: Token) -> Option<Token> {
        if self.unused() {
            None
        } else {
            self.replace(token)
        }
    }

    pub fn peek(&self) -> Option<Token> {
        self.token.clone()
    }

    #[inline]
    pub fn use_once(&mut self) -> &mut Self {
        self.uses += 1;
        self
    }

    pub fn utilize(&mut self) -> Option<Token> {
        self.use_once()
            .peek()
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
    pub(crate) fn mark_unused(&mut self) -> &mut Self {
        self.current_token.set_unused();
        self
    }

    #[inline]
    pub(crate) fn mark_used(&mut self) -> &mut Self {
        self.current_token.use_once();
        self
    }

    #[inline]
    #[allow(dead_code)]
    fn grab_token_from_tokenizer<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
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
    fn grab_if_used<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> Result<&mut Self> {
        // DO NOT REMOVE THIS IF BLOCK
        // BECAUSE IF YOU DO THE TOKENS
        // WILL FALL OFF THE CLIFF BEFORE
        // THEY GET PROCESSED
        if self.current_token.unused() {
            return Ok(self)
        }
        self.replace_used_token(match tokenizer.next_token(stream) {
            Ok(token) => token,
            Err(e) => return Err(Error::from_err(
                Box::new(e),
                ErrorKind::LexerError
            ))
        })
    }

    #[inline]
    #[allow(dead_code)]
    fn get_current_token(&mut self) -> Option<Token> {
        let token = self.current_token.utilize();
        // println!("GRABBED {:?}\n", token);
        token
    }

    #[inline]
    fn peek_current_token(&self) -> Option<Token> {
        let token = self.current_token.peek();
        println!("[{}] {:?}", function_path!(), token);
        println!("[{}] uses: {}\n", function_path!(), self.current_token.uses);
        token
    }

    #[allow(dead_code)]
    fn find_matching_right_round_bracket<'a, 'b: 'a>(
        &mut self,
        lbracket_index: FileIndex,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> Result<Token> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        match self.peek_current_token() {
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
                    self.mark_used();
                    Ok(token.clone())
                } else {
                    Err(Error::new(
                        &format!(
                            "Unmatched bracket {} at {}",
                            bracket,
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

    pub fn parse_top_level_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<FunctionNode> {
        // println!("[{}] Entering\n", function_path!()); 
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
        // println!("[{}] Parsed\n", function_path!());
        Ok(Some(Box::new(FunctionNode::new(
            Box::new(prototype),
            expression
        ))))
    }

    pub fn parse_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        let lhs = self.parse_primary_expression(ltuplemut!(stream, tokenizer))?;
        let mut escaped_from_inner = false;
        match lhs {
            None => Ok(None),
            Some(lhs) => self.parse_binary_operator_rhs_expression(
                lhs,
                Operator::Unknown,
                BinaryOperatorPrecedence::Unknown,
                &mut escaped_from_inner,
                0,
                ltuplemut!(stream, tokenizer)
            )
        }
    }

    pub fn parse_primary_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
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

    pub fn parse_integer_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.peek_current_token());
        println!("[{}] token: {:?}\n", function_path!(), token);
        if let TokenKind::Integer = token.token_kind {
            println!("[{}] integer detected\n", function_path!());
            let rust_integer = match
                token.borrow_span().parse::<IntegerType>()
            {
                Ok(i) => i,
                Err(e) => return Err(Error::from_err(
                    Box::new(e),
                    ErrorKind::ParsingError
                ))
            };
            self.mark_used();
            Ok(Some(Box::new(IntegerNode::new(rust_integer))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_float_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.peek_current_token());
        // println!("[{}] token: {:?}\n", function_path!(), token);
        if let TokenKind::Float = token.token_kind {
            // println!("[{}] float detected\n", function_path!());
            let rust_float = match token.borrow_span().parse::<FloatType>() {
                Ok(f) => f,
                Err(e) => return Err(Error::from_err(
                    Box::new(e),
                    ErrorKind::ParsingError
                ))
            };
            self.mark_used();
            Ok(Some(Box::new(FloatNode::new(rust_float))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_variable_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.peek_current_token());
        // println!("[{}] token: {:?}\n", function_path!(), token);
        if let TokenKind::Identifier = token.token_kind {
            // println!("[{}] identifier detected\n", function_path!());
            let identifier = Box::new(
                IdentifierNode::new(token.borrow_span().to_string())
            );
            self.mark_used();
            Ok(Some(Box::new(VariableExpressionNode::new(identifier))))
        } else {
            Ok(None)
        }
    }

    pub fn parse_binary_operator_rhs_expression<'a, 'b: 'a>(
        &mut self,
        mut lhs: Box<dyn ExprNode>,
        loperator: Operator,
        minimum_operator_precedence: BinaryOperatorPrecedence,
        escaped_from_inner: &mut bool,
        mut depth: usize,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
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

        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let mut loperator = if let Operator::Unknown = loperator {
            let possible_loperator = match self.peek_current_token() {
                Some(o) => o,
                None => {
                    *escaped_from_inner = true;
                    return Ok(Some(lhs))
                }
            };
            match possible_loperator.token_kind {
                TokenKind::Operator {operator} => {
                    self.mark_used();
                    operator
                },
                _ => {
                    *escaped_from_inner = true;
                    return Ok(Some(lhs))
                }
            }
        } else {
            loperator
        };
        depth += 1;

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
            println!(
                "[{}]{} escaped_from_inner: {}",
                function_path!(),
                depth,
                escaped_from_inner
            );
            if *escaped_from_inner {
                let loperator_token = match self.current_token.peek() {
                    Some(t) => t,
                    None => return Ok(Some(lhs))
                };
                loperator = match loperator_token.token_kind {
                    TokenKind::Operator {operator} => operator,
                    _ => return Ok(Some(lhs))
                };
            }
            println!(
                "[{}]{} loperator: {:?}\n",
                function_path!(),
                depth,
                loperator
            );
            let lprecedence: BinaryOperatorPrecedence = loperator.into();
            if lprecedence < minimum_operator_precedence {
                self.mark_unused();
                *escaped_from_inner = true;
                return Ok(Some(lhs));
            }
            let mut rhs = match
                self.parse_primary_expression(ltuplemut!(stream, tokenizer))?
            {
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
            let possible_roperator = match self.peek_current_token() {
                Some(token) => token,
                None => {
                    *escaped_from_inner = true;
                    return up(loperator, lhs, rhs)
                }
            };
            let roperator = match possible_roperator.token_kind {
                TokenKind::Operator {operator} => operator,
                _ => {
                    *escaped_from_inner = true;
                    return up(loperator, lhs, rhs)
                }
            };
            self.mark_used();
            println!(
                "[{}]{} roperator: {:?}\n",
                function_path!(),
                depth,
                roperator
            );
            let rprecedence =
                BinaryOperatorPrecedence::from_operator(roperator);
            if lprecedence < rprecedence {
                *escaped_from_inner = false;
                rhs = ok_none!(self.parse_binary_operator_rhs_expression(
                    rhs,
                    roperator,
                    rprecedence,
                    escaped_from_inner,
                    depth,
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
            println!("[{}]{} new lhs: {}\n", function_path!(), depth, lhs);
        }
    }

    pub fn parse_round_bracket_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.peek_current_token());
        // println!("[{}] token: {:?}\n", function_path!(), token);
        let left_bracket = match token.token_kind {
            TokenKind::Bracket {bracket} => bracket,
            _ => return Ok(None)
        };
        self.mark_used();
        if !matches!(left_bracket.kind, BracketKind::Round) {
            if !left_bracket.side.is_left() {
                return Err(Error::new(
                    &"Mismatched right bracket.",
                    ErrorKind::SyntaxError,
                    None
                ));
            } else {
                return Ok(None);
            }
        }
        // println!("[{}] left bracket verified\n", function_path!());
        let expression = match
            self.parse_expression(ltuplemut!(stream, tokenizer))?
        {
            Some(x) => x,
            None => return Err(Error::new(
                &"Expected expression.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        // println!("[{}] inner expression parsed\n", function_path!());
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = match self.peek_current_token() {
            Some(t) => t,
            None => return Err(Error::new(
                &"Unexpected EOF.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        self.mark_used();
        let right_bracket = match token.token_kind {
            TokenKind::Bracket {bracket} => bracket,
            _ => return Err(Error::new(
                &"Expected round right bracket.",
                ErrorKind::SyntaxError,
                None
            ))
        };
        if !left_bracket.cancels_out(right_bracket) {
            return Err(Error::new(
                &"Incompatible brackets.",
                ErrorKind::SyntaxError,
                None
            ));
        }
        // println!("[{}] right bracket validated\n", function_path!());
        Ok(Some(expression))
    }

    pub fn parse_function_prototype<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<FunctionPrototypeNode> {
        // println!("[{}] Entering\n", function_path!());
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let def_token = ok_none!(self.peek_current_token());
        match def_token.token_kind {
            TokenKind::Keyword {keyword} => match keyword {
                Keyword::Def => (),
                _ => return Ok(None)
            },
            _ => return Ok(None)
        };
        self.mark_used();
        // println!("[{}] def keyword found!\n", function_path!());

        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let function_identifier_token = match self.peek_current_token() {
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
        self.mark_used();
        let function_identifier = match function_identifier_token.token_kind {
            TokenKind::Identifier => Box::new(IdentifierNode::new(
                function_identifier_token.borrow_span().to_string()
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
        // println!(
        //     "[{}] identifier name: {}\n",
        //     function_path!(),
        //     function_identifier.get_identifier()
        // );

        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let lbracket_token = match self.peek_current_token() {
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
        self.mark_used();
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
        // println!("[{}] left bracket found\n", function_path!());

        let mut parameters: Vec<Box<IdentifierNode>> = Vec::new();
        let mut ended = false;

        loop {
            self.grab_if_used(ltuplemut!(stream, tokenizer))?;
            if let Some(parameter_token) = self.peek_current_token() {
                // println!(
                //     "[{}] parameter_token: {:?}\n",
                //     function_path!(),
                //     parameter_token
                // );
                self.mark_used();
                self.grab_if_used(ltuplemut!(stream, tokenizer))?;
                let possible_comma_token = self.peek_current_token();
                // println!(
                //     "[{}] possible_comma_token: {:?}\n",
                //     function_path!(),
                //     possible_comma_token
                // );
                match possible_comma_token {
                    Some(comma_token) => match comma_token.token_kind {
                        TokenKind::Comma => (),
                        TokenKind::Bracket {bracket} =>
                            if LEFT_ROUND_BRACKET.cancels_out(bracket) {
                                ended = true;
                            } else {
                                return Err(Error::new(
                                    &format!(
                                        "Expected a ')' instead of this {} at {}",
                                        bracket,
                                        comma_token.start
                                    ),
                                    ErrorKind::SyntaxError,
                                    None
                                ));
                            },
                        _ => return Err(Error::new(
                            &format!(
                                "Unexpected token after parameter name at {}",
                                comma_token.start
                            ),
                            ErrorKind::SyntaxError,
                            None
                        ))
                    },
                    None => return Err(Error::new(
                        &format!(
                            "Unexpected EOF for function prototype at {}",
                            function_identifier_token.start
                        ),
                        ErrorKind::SyntaxError,
                        None
                    ))
                }
                self.mark_used();
                parameters.push(match parameter_token.token_kind {
                    TokenKind::Identifier => Box::new(IdentifierNode::new(
                        parameter_token.borrow_span().to_string()
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
        // println!("[{}] Parsed\n", function_path!());

        Ok(Some(Box::new(FunctionPrototypeNode::new(
            function_identifier,
            parameters
        ))))
    }

    pub fn parse_function<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
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

    pub fn parse_extern_function<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<ExternFunctionNode> {
        // println!("[{}] Entering\n", function_path!());
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let extern_token = ok_none!(self.peek_current_token());
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
        self.mark_used();
        // println!("[{}] extern keyword found\n", function_path!());

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
        // println!("[{}] Parsed\n", function_path!());
        Ok(Some(Box::new(ExternFunctionNode::new(prototype))))
    }
}
