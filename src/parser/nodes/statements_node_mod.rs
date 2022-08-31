#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Statements node
/// Does not implement node visit because it is not a real node
#[derive(Debug, Clone)]
pub struct StatementsNode {
    pub statements: Vec<Node>,
}

impl StatementsNode {
    pub fn new(statements: Vec<Node>) -> StatementsNode {
        StatementsNode { statements }
    }
}

