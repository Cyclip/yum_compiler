//! Symbol tables store the names of variables and functions, and their
//! associated values, acting as a kind of dictionary or hashmap.
//! The interpreter structure will store a symbol table with no parent,
//! acting as the global symbol table.

use std::collections::HashMap;
use super::{symbols::{Symbol, SymbolType, FunctionSymbol}, builtin};
use crate::{errors::Error, parser::nodes::{Node, ExecuteBuiltinNode}, lexer::tokens::{TokenType, TokenPosition}};

#[derive(Clone, Debug)]
pub struct SymbolTable<'a> {
    pub parent: Option<Box<&'a SymbolTable<'a>>>,
    pub symbols: HashMap<String, Symbol>,
}

/// General SymbolTable implementations =============================================================
impl<'a> SymbolTable<'a> {
    /// Create a new symbol table
    pub fn new(parent: Box<&'a SymbolTable>) -> SymbolTable<'a> {
        SymbolTable {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }

    /// Nicer shortcut for creating a global symbol table
    pub fn new_global() -> SymbolTable<'a> {
        let mut symbol_table = SymbolTable {
            parent: None,
            symbols: HashMap::new(),
        };

        symbol_table.add_builtin_functions();

        symbol_table
    }

    /// Insert a new identifier into the symbol table
    /// If the identifier already exists, it will be overwritten
    pub fn set(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
        println!("{:#?}", self);
    }

    /// Get the value associated with an identifier
    /// If it is not found, it will search the parent symbol table
    /// If it is not found in any symbol tables up to the global symbol table,
    /// it will return None
    pub fn get(&self, identifier: &String) -> Option<Symbol> {
        match self.symbols.get(identifier) {
            Some(symbol) => Some(symbol.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(identifier),
                None => None,
            },
        }
    }

    /// For function calls, set arguments to the function symbol table
    pub fn set_args(&mut self, func_args: &Vec<String>, args: Vec<Symbol>) {
        for i in func_args.iter().zip(args.iter()) {
            self.set(i.0.clone(), i.1.clone());
        }
    }
}


/// Built in functions =============================================================
impl<'a> SymbolTable<'a> {
    /// Add a function to the symbol table
    pub fn add_function(&mut self, name: &str, args: Vec<String>, func: &'static dyn Fn(&mut SymbolTable, &Vec<Node>) -> Result<Symbol, Error>) {
        self.symbols.insert(
            name.to_string(),
            Symbol::new(
                SymbolType::Function(FunctionSymbol::new(
                    name.to_string(),
                    args.clone(),
                    Node::ExecuteBuiltinNode(Box::new(ExecuteBuiltinNode::new(func, None)))
                )),
                TokenPosition::internal()
            )
        );
    }

    /// Add built-in functions to the global symbol table
    fn add_builtin_functions(&mut self) {
        // print function
        self.add_function(
            "print",
            vec!["text".to_string(), "arguments".to_string()],
            &builtin::print::print_func
        );
    }
    
        
}