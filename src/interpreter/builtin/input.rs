use crate::interpreter::symbol_table::SymbolTable;
use crate::interpreter::symbols::{Symbol, SymbolType};
use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::TokenPosition;
use crate::parser::nodes::Node;
use std::io::{stdin,stdout,Write};

pub fn input_func(symbol_table: &mut SymbolTable, args: &Vec<Node>) -> Result<Symbol, Error> {
    let symbol_args = super::evaluate_args(symbol_table, args)?;

    let prefix = symbol_args[0].value.to_string();
    print!("{}", prefix);
    
    let _ = stdout().flush();
    
    let mut line = String::new();
    match stdin().read_line(&mut line) {
        Ok(_) => (),
        Err(e) => return Err(Error::new_runtime(
            ErrorType::IOError, 
            format!("Error reading from stdin: {}", e),
            &symbol_args[0].position
        ))
    }; // including '\n'

    trim_newline(&mut line);

    let symbol = Symbol::new(SymbolType::String(line), TokenPosition::internal());
    Ok(symbol)
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}