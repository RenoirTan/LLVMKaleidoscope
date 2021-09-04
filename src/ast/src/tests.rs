use crate::{nodes::*, prelude::*};


macro_rules! node_types_to_id {
    ( $(,)? ) => {{
        Vec::<$crate::NodeId>::new();
    }};
    ($($node: ident),+ $(,)?) => {{
        use $crate::{
            NodeId,
            node::NodeType
        };
        <[NodeId]>::into_vec(Box::new([$($node::node_id()),+]))
    }};
}


#[test]
fn test_ensure_all_nodes_have_unique_ids() {
    let nodes = node_types_to_id![
        BinaryOperatorNode,
        ExternFunctionNode,
        FloatNode,
        FunctionCallNode,
        FunctionPrototypeNode,
        FunctionNode,
        IdentifierNode,
        IntegerNode,
        Operator,
        UnaryOperatorNode,
        VariableExpressionNode
    ];
    for (lindex, lhs) in nodes.iter().enumerate() {
        for rhs in &nodes[lindex + 1..] {
            if lhs == rhs {
                panic!("Similar node ids! {}", lhs);
            }
        }
    }
}

/*
#[test]
fn test_upcast_expr_node() {
    let expr_node: Box<dyn ExprNode> = Box::new(FloatNode::new(34209.39843));
    let node: Box<dyn Node> = upcast_expr_node(expr_node);
    assert_eq!(node.node_id_of_val(), FloatNode::node_id());
}
*/

#[test]
fn test_reify_node() {
    let unknown: Box<dyn Node> = Box::new(IdentifierNode::new(String::from("name")));
    let concrete: Box<IdentifierNode> = reify_node(unknown).unwrap();
    assert_eq!(concrete.get_identifier(), "name");

    let unknown: Box<dyn Node> = Box::new(Operator::Plus);
    assert!(reify_node::<IdentifierNode>(unknown).is_none());
}

#[test]
fn test_reify_expr_node() {
    let unknown: Box<dyn ExprNode> = Box::new(FloatNode::new(2.718));
    let concrete: Box<FloatNode> = reify_expr_node(unknown).unwrap();
    assert_eq!(concrete.get_value(), 2.718);

    let unknown: Box<dyn ExprNode> = Box::new(IntegerNode::new(55));
    assert!(reify_expr_node::<FloatNode>(unknown).is_none());
}

/*
#[test]
fn test_reify_expr_node_after_upcast() {
    let unknown: Box<dyn ExprNode> = Box::new(VariableExpressionNode::new(
        Box::new(IdentifierNode::new(String::from("__name__")))
    ));
    let unknown = upcast_expr_node(unknown);
    assert_eq!(unknown.node_id_of_val(), VariableExpressionNode::node_id());
    let concrete = reify_node::<VariableExpressionNode>(unknown).unwrap();
    assert_eq!(concrete.get_identifier().get_identifier(), "__name__");
}
*/
