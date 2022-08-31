use crate::errors::{Error, ErrorType};

#[allow(unused_imports)]
use super::bool_to_int;
use super::super::symbols::{Symbol, SymbolType};

impl Symbol {
    pub fn not(&self) -> Result<Symbol, Error> {
        match self.value {
            SymbolType::Integer(a) => Ok(Symbol::new(SymbolType::Integer(bool_to_int(a == 0)), self.position)),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot apply not to {:?}", self.value),
                &self.position
            ))
        }
    }
}