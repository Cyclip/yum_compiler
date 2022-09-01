#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{lexer::tokens::{TokenType, Keyword}};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Unary operation node
#[derive(Debug, Clone)]
pub struct UnaryOpNode {
    pub token: Token,
    pub right: Node,
}

impl UnaryOpNode {
    pub fn new(token: Token, right: Node) -> UnaryOpNode {
        UnaryOpNode {
            token,
            right: right,
        }
    }
}

impl NodeVisit for UnaryOpNode {
    fn visit(&self, symbol_table: &mut crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        let right_symbol = self.right.visit(symbol_table)?;
        match self.token.value {
            TokenType::Keyword(Keyword::Not) => {Ok(right_symbol.not()?)}

            TokenType::Plus => Ok(right_symbol),

            TokenType::Minus => Ok(right_symbol.neg()?),

            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                "Expected unary operation".to_string(), 
                &self.token.position
            ))
        }
    }

    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.token.position
    }
}

impl ToString for UnaryOpNode {
    fn to_string(&self) -> String {
        format!("{} {}", self.token.value, self.right.to_string())
    }
}