use crate::errors::{Error, ErrorType};

#[allow(unused_imports)]
use super::super::symbols::{Symbol, SymbolType};

impl Symbol {
    pub fn power(&self, other: &Symbol) -> Result<Symbol, Error> {
        match (self.value.clone(), other.value.clone()) {
            (SymbolType::Integer(a), SymbolType::Integer(b)) => Ok(Symbol::new(SymbolType::Integer(i32::pow(a, b as u32)), self.position)),
            (SymbolType::Float(a), SymbolType::Float(b)) => Ok(Symbol::new(SymbolType::Float(f32::powf(a, b)), self.position)),
            (SymbolType::Integer(a), SymbolType::Float(b)) => Ok(Symbol::new(SymbolType::Float(f32::powf(a as f32, b)), self.position)),
            (SymbolType::Float(a), SymbolType::Integer(b)) => Ok(Symbol::new(SymbolType::Float(f32::powf(a , b as f32)), self.position)),
            _ => Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot power {:?} and {:?}", self.value, other.value),
                &self.position
            ))
        }
    }
}