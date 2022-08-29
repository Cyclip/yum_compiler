#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// List expression node
#[derive(Debug)]
pub struct ListExprNode {
    pub elements: Vec<Node>,
}

impl ListExprNode {
    pub fn new(elements: Vec<Node>) -> ListExprNode {
        ListExprNode {
            elements,
        }
    }
}

