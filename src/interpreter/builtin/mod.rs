use crate::parser::nodes::Node;
use super::{symbols::Symbol, symbol_table::SymbolTable};
use crate::errors::Error;
use crate::parser::nodes::NodeVisit;

pub mod print;

fn evaluate_args(args: &Vec<Node>) -> Result<Vec<Symbol>, Error> {
    let mut symbol_args = Vec::new();

    for arg in args {
        let symbol = arg.visit(&mut SymbolTable::new_global())?;
        symbol_args.push(symbol);
    }

    Ok(symbol_args)
}