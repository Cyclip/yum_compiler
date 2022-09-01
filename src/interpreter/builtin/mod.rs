use crate::interpreter::symbols::{Symbol, SymbolType};
use crate::errors::Error;
use crate::lexer::tokens::TokenPosition;

pub fn print_func(args: &Vec<Symbol>) -> Result<Symbol, Error> {
    println!("{:?}", &args);
    Ok(Symbol::new(SymbolType::None, TokenPosition::internal()))
}