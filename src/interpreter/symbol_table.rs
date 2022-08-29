//! Symbol tables store the names of variables and functions, and their
//! associated values, acting as a kind of dictionary or hashmap.
//! The interpreter structure will store a symbol table with no parent,
//! acting as the global symbol table.

use std::collections::HashMap;

use super::symbols::Symbol;

pub struct SymbolTable {
    pub parent: Option<Box<SymbolTable>>,
    pub symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    /// Create a new symbol table
    pub fn new(parent: Option<Box<SymbolTable>>) -> SymbolTable {
        SymbolTable {
            parent,
            symbols: HashMap::new(),
        }
    }

    /// Nicer shortcut for creating a global symbol table
    pub fn new_global() -> SymbolTable {
        SymbolTable::new(None)
    }

    /// Insert a new identifier into the symbol table
    /// If the identifier already exists, it will be overwritten
    pub fn insert(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }

    /// Get the value associated with an identifier
    /// If it is not found, it will search the parent symbol table
    /// If it is not found in any symbol tables up to the global symbol table,
    /// it will return None
    pub fn get(&self, identifier: String) -> Option<Symbol> {
        match self.symbols.get(&identifier) {
            Some(symbol) => Some(symbol.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(identifier),
                None => None,
            },
        }
    }
}