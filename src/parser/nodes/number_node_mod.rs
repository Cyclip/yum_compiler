#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Number (int/float) node
#[derive(Debug)]
pub struct NumberNode {
    pub token: Token,
}

impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode { token }
    }
}

