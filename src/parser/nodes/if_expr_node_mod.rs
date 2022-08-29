#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// If statement expression node
#[derive(Debug)]
pub struct IfExprNode {
    condition: Node,
    pub if_true: Node,
    pub if_false: Option<Node>,
}

impl IfExprNode {
    pub fn new(condition: Node, if_true: Node, if_false: Option<Node>) -> IfExprNode {
        IfExprNode {
            condition,
            if_true,
            if_false,
        }
    }
}

