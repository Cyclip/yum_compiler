#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Binary operation node
#[derive(Debug)]
pub struct BinOpNode {
    pub left_node: Node,
    pub op_token: Token,
    pub right_node: Node,
}

impl BinOpNode {
    pub fn new(left_node: Node, op_token: Token, right_node: Node) -> BinOpNode {
        BinOpNode {
            left_node,
            op_token,
            right_node,
        }
    }
}

