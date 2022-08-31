#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{interpreter::symbol_table::SymbolTable, lexer::tokens::TokenPosition};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Function call node
#[derive(Debug, Clone)]
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

impl NodeVisit for FuncCallNode {
    fn visit(&self, _symbol_table: &mut SymbolTable) -> Result<Symbol, Error> {
        unimplemented!()
    }

    fn get_position(&self) -> TokenPosition {
        self.func_node.get_position()
    }
}
