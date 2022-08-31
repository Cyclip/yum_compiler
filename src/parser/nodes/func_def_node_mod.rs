#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::lexer::tokens::TokenPosition;
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};
use crate::interpreter::symbol_table::SymbolTable;

/// Function definition node
#[derive(Debug, Clone)]
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

impl NodeVisit for FuncDefNode {
    fn visit(&self, symbol_table: SymbolTable) -> Result<Symbol, Error> {
        unimplemented!()
    }
    
    fn get_position(&self) -> TokenPosition {
        self.identifier.position
    }
}
