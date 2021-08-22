use kaleidoscope_ast::{
    nodes::{
        FloatNode,
        IntegerNode
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
