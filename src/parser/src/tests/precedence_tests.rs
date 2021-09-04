use crate::precedence::*;


#[test]
fn test_precedence() {
    let bad = BinaryOperatorPrecedence::from_string("fffirj");
    let plus = BinaryOperatorPrecedence::from_string("+");
    let multiply = BinaryOperatorPrecedence::from_string("*");
    assert!(bad < plus);
    assert!(plus < multiply);
}
