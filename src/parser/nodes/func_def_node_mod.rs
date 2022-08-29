#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Function definition node
#[derive(Debug)]
pub struct FuncDefNode {
    pub identifier: Token,
    pub parameters: Vec<Token>,
    pub body: Option<Node>,
}

impl FuncDefNode {
    pub fn new(identifier: Token, parameters: Vec<Token>, body: Option<Node>) -> FuncDefNode {
        FuncDefNode {
            identifier,
            parameters,
            body,
        }
    }
}

