use crate::parser::nodes::Node;
use super::{symbols::Symbol, symbol_table::SymbolTable};
use crate::errors::Error;
use crate::parser::nodes::NodeVisit;

pub mod print;
pub mod input;

fn evaluate_args(symbol_table: &mut SymbolTable, args: &Vec<Node>) -> Result<Vec<Symbol>, Error> {
    let mut symbol_args = Vec::new();

    for arg in args {
        let symbol = arg.visit(symbol_table)?;
        symbol_args.push(symbol);
    }

    Ok(symbol_args)
}