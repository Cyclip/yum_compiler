#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// String node (includes chars)
#[derive(Debug)]
pub struct StringNode {
    pub token: Token,
}

impl StringNode {
    pub fn new(token: Token) -> StringNode {
        StringNode { token }
    }
}

