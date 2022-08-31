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
    fn visit(&self, symbol_table: SymbolTable) -> Result<Symbol, Error> {
        panic!("Not implemented");
        let func_symbol = match self.func_node {
            Node::VarAccessNode(ref node) => node.visit(symbol_table)?,
            _ => return Err(Error::new_runtime(
                ErrorType::InvalidSyntax, 
                format!("Can't call non-function"), 
                &self.get_position()
            )),
        };

        let func_symbol = match func_symbol {
            Symbol::Function(func_symbol) => func_symbol,
            _ => return Err(Error::new_runtime(
                ErrorType::InvalidSyntax, 
                format!("Can't call non-function"), 
                &self.get_position()
            )),
        };

        unimplemented!()
    }

    fn get_position(&self) -> TokenPosition {
        self.func_node.get_position()
    }
}
