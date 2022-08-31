#[allow(unused_imports)]
use super::{Node, NodeVisit};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// List expression node
#[derive(Debug, Clone)]
pub struct ListExprNode {
    pub elements: Vec<Node>,
}

impl ListExprNode {
    pub fn new(elements: Vec<Node>) -> ListExprNode {
        ListExprNode {
            elements,
        }
    }
}

impl NodeVisit for ListExprNode {
    fn visit(&self, _symbol_table: &mut crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        unimplemented!()
    }

    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.elements[0].get_position()
    }
}