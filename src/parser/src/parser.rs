//! The module defining functions to parse a given Kaleidoscope input. This
//! parser can be used to generate Abstract Syntax Trees (ASTs), from which
//! LLVM IR code can be created.

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
    // function_path,
    ok_none,
    return_ok_some
};
use crate::{
    error::{Error, ErrorKind, Result},
    precedence::BinaryOperatorPrecedence
};

/// The return type of most parser functions in [`Parser`].
pub type ParseResult<T> = Result<Option<Box<T>>>;

/// The token manager struct that determines when a [`Token`] should be released
/// for the parser's use or when a new token should be read from the stream.
/// This struct is marked private because the functionality of this struct
/// is only useful when applied in conjunction with [`Parser`]'s methods.
#[derive(Clone, Debug, Default)]
struct ParserToken {
    pub token: Option<Token>,
    pub uses: usize
}

#[allow(unused)]
impl ParserToken {
    /// Create a new [`ParserToken`] manager.
    pub fn new(token: Token) -> Self {
        Self {token: Some(token), uses: 0}
    }

    /// Check if the [`Token`] stored inside has been used at least once or
    /// not, if there is a token inside at all. The reason I added the second
    /// condition is because the manager object only knows that the token
    /// should be replaced when a fresh token is required; if there is no 
    /// token in the manager, then a new token should be supplied so that
    /// when the [`Parser`] requests a new token, that token can be used.
    #[inline]
    pub fn unused(&self) -> bool {
        self.uses < 1 && self.token.is_some()
    }

    /// Mark the token inside as unused. However, [`ParserToken::unused`] will
    /// still return true if the token inside the struct does not exist.
    #[inline]
    pub fn set_unused(&mut self) -> &mut Self {
        self.uses = 0;
        self
    }

    /// Replace the token in the [`ParserToken`] manager with a new token,
    /// resetting the use count back to 0, returning the previously-held
    /// token.
    // 
    /// See also: [`ParserToken::replace_used`].
    pub fn replace(&mut self, token: Token) -> Option<Token> {
        // println!("[{}] new token: {:?}\n", function_path!(), token);
        let original = self.token.take();
        self.token = Some(token);
        self.uses = 0; // Do not remove this line
        original
    }

    /// Replace the token in the [`ParserToken`] manager with a new token,
    /// resetting the use count back to 0. However, if the old token hasn't
    /// been used before, the old token gets discarded, returning [`None`]
    /// in the process.
    /// 
    /// See also: [`ParserToken::replace`].
    #[inline]
    pub fn replace_used(&mut self, token: Token) -> Option<Token> {
        if self.unused() {
            None
        } else {
            self.replace(token)
        }
    }

    /// Peek at the token currently inside. This method clones the token
    /// instead of returning a reference to avoid violating rust's
    /// (im)mutable borrowing rules.
    pub fn peek(&self) -> Option<Token> {
        self.token.clone()
    }

    /// Increase the number of uses by one. This marks the token as "stale"
    /// or used.
    #[inline]
    pub fn use_once(&mut self) -> &mut Self {
        self.uses += 1;
        self
    }

    /// Clone the token in the manager and mark it as used.
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

/// The parser struct that converts a Kaleidoscope program into an Abstract
/// Syntax Tree.
pub struct Parser {
    current_token: ParserToken
}

impl Parser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self {
            current_token: Default::default()
        }
    }

    /// Replace the token the parser is currently reading with another one.
    pub fn next_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token.replace(token);
        Ok(self)
    }

    /// Replace the token the parser is currently reading with another one,
    /// given that it has been used or there was no token in the first place.
    pub fn replace_used_token(&mut self, token: Token) -> Result<&mut Self> {
        self.current_token.replace_used(token);
        Ok(self)
    }

    /// Mark the token in the manager as unused.
    #[inline]
    pub(crate) fn mark_unused(&mut self) -> &mut Self {
        self.current_token.set_unused();
        self
    }

    /// Mark the token in the manager as used.
    #[inline]
    pub(crate) fn mark_used(&mut self) -> &mut Self {
        self.current_token.use_once();
        self
    }

    /// Replace the token with a new token from the stream and tokenizer.
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

    /// Pull a new token from the tokenizer and file stream if the token
    /// stored by the parser has been used.
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

    /// Get the current token being stored, marking as used in the process.
    #[inline]
    #[allow(dead_code)]
    fn get_current_token(&mut self) -> Option<Token> {
        let token = self.current_token.utilize();
        // println!("GRABBED {:?}\n", token);
        token
    }

    /// Peek at the current token being stored, without marking it as used.
    #[inline]
    pub fn peek_current_token(&self) -> Option<Token> {
        let token = self.current_token.peek();
        // println!("[{}] {:?}", function_path!(), token);
        // println!("[{}] uses: {}\n", function_path!(), self.current_token.uses);
        token
    }

    /// Helper code that finds a right bracket that cancels out a left bracket.
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

    /// Converts an expression into an anonymous function.
    pub fn parse_top_level_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        self.parse_expression(ltuplemut!(stream, tokenizer))
        /* let expression = match
            self.parse_expression(ltuplemut!(stream, tokenizer))?
        {
            Some(ex) => ex,
            None => return Ok(None)
        };
        let prototype = FunctionPrototypeNode::new(
            Box::new(IdentifierNode::new(String::from(""))),
            Vec::new()
        );
        Ok(Some(Box::new(FunctionNode::new(
            Box::new(prototype),
            expression
        )))) */
    }

    /// Parse an expression.
    pub fn parse_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        let lhs = self.parse_primary_expression(ltuplemut!(stream, tokenizer))?;
        log::trace!("primary expression parsed");
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

    /// Parse a so-called "primary" expression. You can think of primary
    /// expressions as simple expressions, which means that parsing it is
    /// relatively trivial. This category includes integers, floats, variables
    /// and other expressions wrapped around two corresponding round brackets.
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

    /// Parse an integer expression.
    pub fn parse_integer_expression<'a, 'b: 'a>(
        &mut self,
        ltuplemut!(stream, tokenizer): LexerTupleMut<'a, 'b>
    ) -> ParseResult<dyn ExprNode> {
        self.grab_if_used(ltuplemut!(stream, tokenizer))?;
        let token = ok_none!(self.peek_current_token());
        // println!("[{}] token: {:?}\n", function_path!(), token);
        if let TokenKind::Integer = token.token_kind {
            // println!("[{}] integer detected\n", function_path!());
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

    /// Parse a float expression.
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

    /// Parse a variable expression.
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

    /// Parse an expression wrapped inside 2 round brackets.
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

    /// Parse a binary operator expression. This is similar to simple math
    /// equations like `1 + 1` or `5 * 3`.
    pub fn parse_binary_operator_rhs_expression<'a, 'b: 'a>(
        &mut self,
        mut lhs: Box<dyn ExprNode>,
        mut loperator: Operator,
        minimum_operator_precedence: BinaryOperatorPrecedence,
        escaped_from_inner: &mut bool,
        depth: usize,
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
        if let Operator::Unknown = loperator {
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
                    loperator = operator;
                },
                _ => {
                    *escaped_from_inner = true;
                    return Ok(Some(lhs))
                }
            }
        }
        let mut roperator = Operator::Unknown;

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
            // println!(
            //     "[{}]{} escaped_from_inner: {}",
            //     function_path!(),
            //     depth,
            //     escaped_from_inner
            // );
            if *escaped_from_inner {
                let loperator_token = match self.peek_current_token() {
                    Some(t) => t,
                    None => return Ok(Some(lhs))
                };
                loperator = match loperator_token.token_kind {
                    TokenKind::Operator {operator} => {
                        self.mark_used();
                        operator
                    },
                    _ => return Ok(Some(lhs))
                };
            } else if !matches!(roperator, Operator::Unknown) {
                loperator = roperator;
            }
            // println!(
            //     "[{}]{} loperator: {:?}\n",
            //     function_path!(),
            //     depth,
            //     loperator
            // );
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
            roperator = match possible_roperator.token_kind {
                TokenKind::Operator {operator} => operator,
                _ => {
                    *escaped_from_inner = true;
                    return up(loperator, lhs, rhs)
                }
            };
            self.mark_used();
            // println!(
            //     "[{}]{} roperator: {:?}\n",
            //     function_path!(),
            //     depth,
            //     roperator
            // );
            let rprecedence =
                BinaryOperatorPrecedence::from_operator(roperator);
            if lprecedence < rprecedence {
                *escaped_from_inner = false;
                rhs = ok_none!(self.parse_binary_operator_rhs_expression(
                    rhs,
                    roperator,
                    rprecedence,
                    escaped_from_inner,
                    depth+1,
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
            // println!("[{}]{} new lhs: {}\n", function_path!(), depth, lhs);
        }
    }

    /// Parse a function prototype.
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

    /// Parse a function definition.
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

    /// Parse an extern function declaration.
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
