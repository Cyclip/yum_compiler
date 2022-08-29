#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Variable assignment node
#[derive(Debug)]
pub struct VarAssignmentNode {
    pub identifier: Token,
    pub value: Node,
}

impl VarAssignmentNode {
    pub fn new(identifier: Token, value: Node) -> VarAssignmentNode {
        VarAssignmentNode {
            identifier,
            value,
        }
    }
}

