#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Variable assignment node
#[derive(Debug)]
pub struct VarAccessNode {
    pub identifier: Token,
}



impl VarAccessNode {
    pub fn new(identifier: Token) -> VarAccessNode {
        VarAccessNode { identifier }
    }
}

