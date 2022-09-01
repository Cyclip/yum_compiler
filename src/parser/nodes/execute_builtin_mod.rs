use std::fmt::Debug;

#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{interpreter::symbol_table::SymbolTable, lexer::tokens::{Keyword, TokenPosition}};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::{Token, TokenType}, errors::{Error, ErrorType}};

/// Execute rust-based built-in functions
#[derive(Clone)]
pub struct ExecuteBuiltinNode {
    pub func: &'static dyn Fn(&mut SymbolTable, &Vec<Node>) -> Result<Symbol, Error>,
    pub args: Option<Vec<Node>>,
}

impl ExecuteBuiltinNode {
    pub fn new(func: &'static dyn Fn(&mut SymbolTable, &Vec<Node>) -> Result<Symbol, Error>, args: Option<Vec<Node>>) -> ExecuteBuiltinNode {
        ExecuteBuiltinNode {
            func,
            args,
        }
    }

    /// Evaluate all argument nodes into a symbol vector
    fn evaluate_arguments(&self, symbol_table: &mut SymbolTable) -> Result<Vec<Symbol>, Error> {
        let mut args: Vec<Symbol> = Vec::new();

        for arg in self.args.as_ref().expect("Arguments are not defined") {
            args.push(arg.visit(symbol_table)?);
        };

        Ok(args)
    }
}

impl NodeVisit for ExecuteBuiltinNode {
    fn visit(&self, symbol_table: &mut SymbolTable) -> Result<Symbol, Error> {
        (self.func)(symbol_table, &self.args.as_ref().expect("Arguments are not defined"))
    }

    fn get_position(&self) -> TokenPosition {
        TokenPosition::internal()
    }
}

impl Debug for ExecuteBuiltinNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExecuteBuiltinNode")
    }
}

impl ToString for ExecuteBuiltinNode {
    fn to_string(&self) -> String {
        "ExecuteBuiltinNode".to_string()
    }
}