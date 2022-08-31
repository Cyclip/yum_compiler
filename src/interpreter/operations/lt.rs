use crate::errors::{Error, ErrorType};

#[allow(unused_imports)]
use super::bool_to_int;
use super::super::symbols::{Symbol, SymbolType};

impl Symbol {
    pub fn lt(&self, other: &Symbol) -> Result<Symbol, Error> {
        match (self.value, other.value) {
            (SymbolType::Integer(a), SymbolType::Integer(b)) => Ok(Symbol::new(SymbolType::Integer(bool_to_int(a < b)), self.position)),
            (SymbolType::Float(a), SymbolType::Float(b)) => Ok(Symbol::new(SymbolType::Integer(bool_to_int(a < b)), self.position)),
            (SymbolType::Integer(a), SymbolType::Float(b)) => Ok(Symbol::new(SymbolType::Integer(bool_to_int((a as f32) < b)), self.position)),
            (SymbolType::Float(a), SymbolType::Integer(b)) => Ok(Symbol::new(SymbolType::Integer(bool_to_int(a < b as f32)), self.position)),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot add {:?} and {:?}", self.value, other.value),
                &self.position
            ))
        }
    }
}