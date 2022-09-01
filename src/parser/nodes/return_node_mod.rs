#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::interpreter::symbols::SymbolType;
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Return statement node
#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub value: Option<Node>,
}

impl ReturnNode {
    pub fn new(value: Option<Node>) -> ReturnNode {
        ReturnNode {
            value: value,
        }
    }
}

impl NodeVisit for ReturnNode {
    fn visit(&self, _symbol_table: &mut crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        match self.value {
            Some(ref value) => value.visit(_symbol_table),
            None => Ok(Symbol::new(SymbolType::None, self.get_position()))
        }
    }

    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.value.as_ref().unwrap().get_position()
    }
}

impl ToString for ReturnNode {
    fn to_string(&self) -> String {
        format!("ReturnNode")
    }
}