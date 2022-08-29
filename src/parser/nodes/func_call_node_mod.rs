#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Function call node
#[derive(Debug)]
pub struct FuncCallNode {
    pub func_node: Node,
    pub args: Vec<Node>,
}

impl FuncCallNode {
    pub fn new(func_node: Node, args: Vec<Node>) -> FuncCallNode {
        FuncCallNode {
            func_node,
            args,
        }
    }
}

