#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{lexer::tokens::TokenType, interpreter::symbols::SymbolType};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// String node (includes chars)
#[derive(Debug, Clone)]
pub struct StringNode {
    pub token: Token,
}

impl StringNode {
    pub fn new(token: Token) -> StringNode {
        StringNode { token }
    }
}

impl NodeVisit for StringNode {
    fn visit(&self, _symbol_table: &mut crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        match self.token.value.clone() {
            TokenType::String(s) => Ok(Symbol::new(SymbolType::String(s), self.get_position())),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                "Expected string".to_string(), 
                &self.token.position
            ))
        }
    }

    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.token.position
    }
}

impl ToString for StringNode {
    fn to_string(&self) -> String {
        format!("{}", self.token.value)
    }
}