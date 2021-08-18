use kaleidoscope_ast::{
    nodes::IntegerNode,
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

#[test]
fn test_parse_integer() {
    let mut parser = Parser::new();
    let mut stream = FileStream::from("420");
    let mut tokenizer = Tokenizer::new();
    let expression = parser.parse_integer_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    let node = reify_expr_node::<IntegerNode>(expression).unwrap();
    assert_eq!(node.get_value(), 420);
}
