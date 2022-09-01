use std::fmt::Debug;

#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{interpreter::symbol_table::SymbolTable, lexer::tokens::{Keyword, TokenPosition}};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::{Token, TokenType}, errors::{Error, ErrorType}};

/// Execute rust-based built-in functions
#[derive(Clone)]
pub struct ExecuteBuiltinNode {
    pub func: fn(&Vec<Symbol>) -> Result<Symbol, Error>,
    pub args: Option<Vec<Node>>,
}

impl ExecuteBuiltinNode {
    pub fn new(func: fn(&Vec<Symbol>) -> Result<Symbol, Error>, args: Option<Vec<Node>>) -> ExecuteBuiltinNode {
        ExecuteBuiltinNode {
            func,
            args,
        }
    }

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
        let args = self.evaluate_arguments(symbol_table)?;
        (self.func)(&args)
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