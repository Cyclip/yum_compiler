#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{lexer::tokens::{TokenType, Keyword}, interpreter::symbols::SymbolType};
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
    fn visit(&self, symbol_table: crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        let right_symbol = self.right.visit(symbol_table)?;
        match self.token.value {
            TokenType::Keyword(Keyword::Not) => {
                match right_symbol.value {
                    SymbolType::Integer(1) => Ok(Symbol::new(SymbolType::Integer(0), self.get_position())),
                    SymbolType::Integer(0) => Ok(Symbol::new(SymbolType::Integer(1), self.get_position())),
                    _ => Err(Error::new_runtime(
                        ErrorType::InvalidOperation,
                        format!("Invalid operation '{:?}'", self.token.value),
                        &self.token.position,
                    )),
                }
            }

            TokenType::Plus => Ok(right_symbol),

            TokenType::Minus => Ok(right_symbol.neg()),

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