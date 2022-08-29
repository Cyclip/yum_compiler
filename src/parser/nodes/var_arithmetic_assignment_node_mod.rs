#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Variable arithmetic assignment node
#[derive(Debug)]
pub struct VarArithmeticAssignmentNode {
    pub identifier: Token,
    pub op_token: Token,
    pub value: Node,
}

impl VarArithmeticAssignmentNode {
    pub fn new(identifier: Token, op_token: Token, value: Node) -> VarArithmeticAssignmentNode {
        VarArithmeticAssignmentNode {
            identifier,
            op_token,
            value,
        }
    }
}

