#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{interpreter::{symbol_table::SymbolTable, symbols::SymbolType}, lexer::tokens::TokenPosition};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Assertion node
#[derive(Debug, Clone)]
pub struct AssertNode {
    pub condition: Node,
}

impl AssertNode {
    pub fn new(condition: Node) -> AssertNode {
        AssertNode {
            condition,
        }
    }
}

impl NodeVisit for AssertNode {
    fn visit(&self, symbol_table: SymbolTable) -> Result<Symbol, Error> {
        let condition_symbol = self.condition.visit(symbol_table)?;
        
        match condition_symbol.value {
            SymbolType::Integer(value) => {
                if value == 0 {
                    return Err(Error::new_runtime(
                        ErrorType::AssertError, 
                        "Assertion failed".to_string(), 
                        &self.get_position()
                    ));
                }
            }
            _ => return Err(Error::new_runtime(
                    ErrorType::AssertError, 
                    "Assertion failed".to_string(), 
                    &self.get_position()
                ))
        };

        Ok(condition_symbol)
        
    }

    fn get_position(&self) -> TokenPosition {
        self.condition.get_position()
    }
}