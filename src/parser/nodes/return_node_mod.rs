#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Return statement node
#[derive(Debug)]
pub struct ReturnNode {
    pub value: Option<Node>,
}

impl ReturnNode {
    pub fn new(value: Option<Node>) -> ReturnNode {
        ReturnNode {
            value: value,
        }
    }
}

