#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::lexer::tokens::TokenType;
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Number (int/float) node
#[derive(Debug, Clone)]
pub struct NumberNode {
    pub token: Token,
}

impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode { token }
    }
}

impl NodeVisit for NumberNode {
    fn visit(&self, symbol_table: crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        match self.token.value {
            TokenType::Integer(i) => Ok(Symbol::Integer(i)),
            TokenType::Float(f) => Ok(Symbol::Float(f)),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                "Expected number".to_string(), 
                &self.token.position
            ))
        }
    }
    
    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.token.position
    }
}