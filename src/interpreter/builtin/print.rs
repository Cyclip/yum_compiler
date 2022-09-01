use crate::interpreter::symbol_table::SymbolTable;
use crate::interpreter::symbols::{Symbol, SymbolType};
use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::TokenPosition;
use crate::parser::nodes::Node;
use crate::parser::nodes::NodeVisit;

/// Print a formatted string to the console.
/// Examples:
/// INPUT: print("Hello, {}!", "world")
/// OUTPUT: Hello, world!
/// 
/// INPUT: print("{}, {}!", "Hello", "world")
/// OUTPUT: Hello, world!
pub fn print_func(symbol_table: &mut SymbolTable, args: &Vec<Node>) -> Result<Symbol, Error> {
    let symbol_args = super::evaluate_args(args)?;
    println!("\n\n\nFunction called. Symbol table: {:#?}\n\nArgs: {:#?}\n\n\n", symbol_table, symbol_args);

    let text = match symbol_args[0].value {
        SymbolType::String(ref text) => text.clone(),
        _ => return Err(Error::new_runtime(
            ErrorType::TypeError, 
            format!("Cannot print non-string {:?}", symbol_args[0]),
            &symbol_args[0].position
        ))
    };

    let format_arguments: Vec<String> = symbol_args[1..].iter()
        .map(|x| x.value.to_string()).collect();

    let final_string = match format_string(text, format_arguments) {
        Ok(x) => x,
        Err(e) => return Err(Error::new_runtime(
            ErrorType::TypeError, 
            e,
            &symbol_args[0].position
        ))
    };

    println!("{}", final_string);

    Ok(Symbol::new(SymbolType::None, TokenPosition::internal()))
}

fn get_position(args: &Vec<Node>) -> TokenPosition {
    args[0].get_position()
}

fn format_string(text: String, args: Vec<String>) -> Result<String, String> {
    if !same_size(&text, &args) {
        return Err(format!("Number of format arguments does not match number of format specifiers in string"));
    };

    let mut split_text_iter = text.split("{}");
    let mut args_iter = args.iter();

    let mut final_string = String::new();

    for i in split_text_iter.zip(args_iter) {
        final_string += &(i.0.to_owned() + i.1);
    };

    Ok(final_string)
}

fn same_size(text: &String, args: &Vec<String>) -> bool {
    let split_text_iter = text.split("{}").count();
    let args_iter = args.len();

    split_text_iter == args_iter + 1
}