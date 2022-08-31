use crate::errors::{Error, ErrorType};

#[allow(unused_imports)]
use super::super::symbols::{Symbol, SymbolType};

impl Symbol {
    pub fn neg(&self) -> Result<Symbol, Error> {
        match self.value {
            SymbolType::Integer(a) => Ok(Symbol::new(SymbolType::Integer(-a), self.position)),
            SymbolType::Float(a) => Ok(Symbol::new(SymbolType::Float(-a), self.position)),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot apply neg to {:?}", self.value),
                &self.position
            ))
        }
    }
}