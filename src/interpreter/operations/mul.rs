use crate::errors::{Error, ErrorType};

#[allow(unused_imports)]
use super::super::symbols::{Symbol, SymbolType};

impl Symbol {
    pub fn mul(&self, other: &Symbol) -> Result<Symbol, Error> {
        match (self.value.clone(), other.value.clone()) {
            (SymbolType::Integer(a), SymbolType::Integer(b)) => Ok(Symbol::new(SymbolType::Integer(a * b), self.position)),
            (SymbolType::Float(a), SymbolType::Float(b)) => Ok(Symbol::new(SymbolType::Float(a * b), self.position)),
            (SymbolType::Integer(a), SymbolType::Float(b)) => Ok(Symbol::new(SymbolType::Float(a as f32 * b), self.position)),
            (SymbolType::Float(a), SymbolType::Integer(b)) => Ok(Symbol::new(SymbolType::Float(a * b as f32), self.position)),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot multiply {:?} and {:?}", self.value, other.value),
                &self.position
            ))
        }
    }
}