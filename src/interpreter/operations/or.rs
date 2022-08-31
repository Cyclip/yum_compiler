use crate::errors::{Error, ErrorType};

#[allow(unused_imports)]
use super::bool_to_int;
use super::super::symbols::{Symbol, SymbolType};

impl Symbol {
    pub fn or(&self, other: &Symbol) -> Result<Symbol, Error> {
        match (self.value, other.value) {
            (SymbolType::Integer(a), SymbolType::Integer(b)) => Ok(Symbol::new(SymbolType::Integer(bool_to_int(a == 1 || b == 1)), self.position)),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot add {:?} and {:?}", self.value, other.value),
                &self.position
            ))
        }
    }
}