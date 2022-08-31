#[allow(unused_imports)]
use super::{Node, NodeVisit, get_name_as_string};
use crate::lexer::tokens::TokenType;
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Variable assignment node
#[derive(Debug, Clone)]
pub struct VarAccessNode {
    pub identifier: Token,
}

impl VarAccessNode {
    pub fn new(identifier: Token) -> VarAccessNode {
        VarAccessNode { identifier }
    }
}

impl NodeVisit for VarAccessNode {
    fn visit(&self, symbol_table: crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        match symbol_table.get(get_name_as_string(self.identifier)?) {
            Some(symbol) => Ok(symbol.clone()),
            None => Err(Error::new_runtime(
                ErrorType::UndefinedVariable, 
                format!("Undefined variable '{:?}'", self.identifier.value), 
                &self.identifier.position
            ))
        }
    }

    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.identifier.position
    }
}