use kaleidoscope_ast::{
    nodes::{
        BinaryOperatorNode,
        FloatNode,
        IdentifierNode,
        IntegerNode,
        Operator,
        VariableExpressionNode
    },
    node::{
        reify_expr_node,
        reify_expr_node_ref
    }
};
use kaleidoscope_lexer::{
    ltuplemut,
    tokenizer::{
        FileStream,
        Tokenizer
    }
};
use kaleidoscope_macro::function_name;
use crate::parser::Parser;

#[allow(dead_code)]
fn print_tokenizer<'a>(tokenizer: Tokenizer, stream: FileStream<'a>) {
    let mut iterable = tokenizer.to_iter(stream);
    for token in &mut iterable {
        println!("{:#?}", token);
    }
}

#[inline]
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
fn test_binop_0() {
    let (mut parser, mut stream, mut tokenizer) = get_parser(
        "1 + 2"
    );
    let expression = parser.parse_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    println!("{}: {}", function_name!(), expression);

    let node = reify_expr_node::<BinaryOperatorNode>(expression).unwrap();
    assert_eq!(*node.get_operator(), Operator::Plus);
        let left = reify_expr_node_ref::<IntegerNode>(node.get_first())
            .unwrap();
        assert_eq!(left.get_value(), 1);
        let right = reify_expr_node_ref::<IntegerNode>(node.get_second())
            .unwrap();
        assert_eq!(right.get_value(), 2);
}

#[test]
fn test_binop_1() {
    let (mut parser, mut stream, mut tokenizer) = get_parser(
        "1 + 2 * 3"
    );
    let expression = parser.parse_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    println!("{}: {}", function_name!(), expression);

    let node = reify_expr_node::<BinaryOperatorNode>(expression).unwrap();
    assert_eq!(*node.get_operator(), Operator::Plus);
        let left = reify_expr_node_ref::<IntegerNode>(node.get_first())
            .unwrap();
        assert_eq!(left.get_value(), 1);
        let right_node = reify_expr_node_ref::<BinaryOperatorNode>(
            node.get_second()
        ).unwrap();
        assert_eq!(*right_node.get_operator(), Operator::Multiply);
            let right_left = reify_expr_node_ref::<IntegerNode>(
                right_node.get_first()
            ).unwrap();
            assert_eq!(right_left.get_value(), 2);
            let right_right = reify_expr_node_ref::<IntegerNode>(
                right_node.get_second()
            ).unwrap();
            assert_eq!(right_right.get_value(), 3); 
}

#[test]
fn test_binop_2() {
    let (mut parser, mut stream, mut tokenizer) = get_parser(
        "1 + 2 * 3 / 4"
    );
    let expression = parser.parse_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    println!("{}: {}", function_name!(), expression);

    let node = reify_expr_node::<BinaryOperatorNode>(expression).unwrap();
    assert_eq!(*node.get_operator(), Operator::Plus);
        let left = reify_expr_node_ref::<IntegerNode>(node.get_first())
            .unwrap();
        assert_eq!(left.get_value(), 1);
        let right_node = reify_expr_node_ref::<BinaryOperatorNode>(
            node.get_second()
        ).unwrap();
        assert_eq!(*right_node.get_operator(), Operator::Divide);
            let right_left_node = reify_expr_node_ref::<BinaryOperatorNode>(
                right_node.get_first()
            ).unwrap();
            assert_eq!(*right_left_node.get_operator(), Operator::Multiply);
                let right_left_left = reify_expr_node_ref::<IntegerNode>(
                    right_left_node.get_first()
                ).unwrap();
                assert_eq!(right_left_left.get_value(), 2);
                let right_left_right = reify_expr_node_ref::<IntegerNode>(
                    right_left_node.get_second()
                ).unwrap();
                assert_eq!(right_left_right.get_value(), 3);
            let right_right = reify_expr_node_ref::<IntegerNode>(
                right_node.get_second()
            ).unwrap();
            assert_eq!(right_right.get_value(), 4);
}

#[test]
fn test_binop_3() {
    let (mut parser, mut stream, mut tokenizer) = get_parser(
        "1 + 2 * 3 / 4 - 5"
    );
    let expression = parser.parse_expression(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    println!("{}: {}", function_name!(), expression);

    let node = reify_expr_node::<BinaryOperatorNode>(expression).unwrap();
    let left_node = reify_expr_node_ref::<BinaryOperatorNode>(node.get_first())
        .unwrap();
    assert_eq!(*left_node.get_operator(), Operator::Plus);
        let left_left = reify_expr_node_ref::<IntegerNode>(
            left_node.get_first()
        ).unwrap();
        assert_eq!(left_left.get_value(), 1);
        let left_right_node = reify_expr_node_ref::<BinaryOperatorNode>(
            left_node.get_second()
        ).unwrap();
        assert_eq!(*left_right_node.get_operator(), Operator::Divide);
            let left_right_left_node =
                reify_expr_node_ref::<BinaryOperatorNode>(
                    left_right_node.get_first()
                ).unwrap();
            assert_eq!(
                *left_right_left_node.get_operator(),
                Operator::Multiply
            );
                let left_right_left_left = reify_expr_node_ref::<IntegerNode>(
                    left_right_left_node.get_first()
                ).unwrap();
                assert_eq!(left_right_left_left.get_value(), 2);
                let left_right_left_right = reify_expr_node_ref::<IntegerNode>(
                    left_right_left_node.get_second()
                ).unwrap();
                assert_eq!(left_right_left_right.get_value(), 3);
            let left_right_right = reify_expr_node_ref::<IntegerNode>(
                left_right_node.get_second()
            ).unwrap();
            assert_eq!(left_right_right.get_value(), 4);
        let right = reify_expr_node_ref::<IntegerNode>(node.get_second()).unwrap();
        assert_eq!(right.get_value(), 5);
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

#[test]
fn test_extern_function() {
    let (mut parser, mut stream, mut tokenizer) = get_parser(
        "extern def iconv(cd, inbuf, inbytesleft, outbuf, outbytesleft)"
    );
    let external = parser.parse_extern_function(
        ltuplemut!(&mut stream, &mut tokenizer)
    ).unwrap().unwrap();
    let prototype = external.get_prototype();
    assert_eq!(prototype.get_identifier().get_identifier(), "iconv");
    assert_eq!(
        prototype.get_parameters(),
        ["cd", "inbuf", "inbytesleft", "outbuf", "outbytesleft"]
            .iter()
            .map(|s| Box::new(IdentifierNode::new(s.to_string())))
            .collect::<Vec<Box<IdentifierNode>>>()
    );
}
