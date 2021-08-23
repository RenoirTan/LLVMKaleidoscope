use kaleidoscope_ast::{
    nodes::{
        FloatNode,
        IdentifierNode,
        IntegerNode,
        VariableExpressionNode
    },
    node::reify_expr_node
};
use kaleidoscope_lexer::{
    ltuplemut,
    tokenizer::{
        FileStream,
        Tokenizer
    }
};
use crate::parser::Parser;

fn get_parser<'a>(input: &'a str) -> (Parser, FileStream<'a>, Tokenizer) {
    (Parser::new(), FileStream::from(input), Tokenizer::new())
}

#[test]
fn test_parse_integer() {
    let (mut parser, mut stream, mut tokenizer) = get_parser("420");
    let expression = parser.parse_integer_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    let node = reify_expr_node::<IntegerNode>(expression).unwrap();
    assert_eq!(node.get_value(), 420);
}

#[test]
fn test_parse_float() {
    let (mut parser, mut stream, mut tokenizer) = get_parser("3.8");
    let expression = parser.parse_float_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    let node = reify_expr_node::<FloatNode>(expression).unwrap();
    assert_eq!(node.get_value(), 3.8);
}

#[test]
fn test_parse_variable_expression() {
    let (mut parser, mut stream, mut tokenizer) = get_parser("var1");
    let expression = parser.parse_variable_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    let node = reify_expr_node::<VariableExpressionNode>(expression).unwrap();
    assert_eq!(node.get_identifier().get_identifier(), "var1");
}

#[test]
fn test_round_bracket() {
    let (mut parser, mut stream, mut tokenizer) = get_parser("(5.0)");
    let expression = parser.parse_round_bracket_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    let node = reify_expr_node::<FloatNode>(expression).unwrap();
    assert_eq!(node.get_value(), 5.0);
}

#[test]
fn test_function_prototype() {
    let (mut parser, mut stream, mut tokenizer) = get_parser("def pow(a, b)");
    let prototype = parser.parse_function_prototype(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    assert_eq!(prototype.get_identifier().get_identifier(), "pow");
    assert_eq!(
        prototype.get_parameters(),
        ["a", "b"]
            .iter()
            .map(|s| Box::new(IdentifierNode::new(s.to_string())))
            .collect::<Vec<Box<IdentifierNode>>>()
    );
}
