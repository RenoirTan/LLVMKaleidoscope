use std::fmt;

type IdInner = u32;

/// An ID for each node type.
///
/// The current map for each type:
/// 0: ???,
/// 
/// 1: ???,
/// 
/// 2: Identifier,
/// 
/// 3: Bare Operator,
/// 
/// 4: Integer,
/// 
/// 5: Float,
/// 
/// 6: Unary Operator,
/// 
/// 7: Binary Operator,
/// 
/// 8: Ternary Operator,
/// 
/// 9: Quaternary Operator,
/// 
/// 10: Quinary Operator,
/// 
/// 11: Function Call,
/// 
/// 12: Function Prototype,
/// 
/// 13: Statement Body,
/// 
/// 14: Function Definition
/// 
#[derive(Copy, Clone, Debug)]
pub struct NodeId {
    numeric: IdInner
}

impl NodeId {
    pub fn new(id: IdInner) -> Self {
        Self {numeric: id}
    }

    pub fn get_id(&self) -> IdInner {
        self.numeric
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", self.numeric)
    }
}