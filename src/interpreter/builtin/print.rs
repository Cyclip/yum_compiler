use crate::interpreter::symbol_table::SymbolTable;
use crate::interpreter::symbols::{Symbol, SymbolType};
use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::TokenPosition;
use crate::parser::nodes::Node;

/// Print a formatted string to the console.
/// Examples:
/// INPUT: print("Hello, {}!", "world")
/// OUTPUT: Hello, world!
/// 
/// INPUT: print("{}, {}!", "Hello", "world")
/// OUTPUT: Hello, world!
pub fn print_func(symbol_table: &mut SymbolTable, args: &Vec<Node>) -> Result<Symbol, Error> {
    let symbol_args = super::evaluate_args(symbol_table, args)?;

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

/// Format a string with the given arguments
fn format_string(text: String, args: Vec<String>) -> Result<String, String> {
    if !same_size(&text, &args) {
        return Err(format!("Number of format arguments does not match number of format specifiers in string"));
    };

    let split_text_iter = text.split("{}");
    let args_iter = args.iter();

    let mut final_string = String::new();

    for i in split_text_iter.zip(args_iter) {
        final_string += &(i.0.to_owned() + i.1);
    };

    Ok(final_string)
}


/// Check if the number of format specifiers in the string is the same as the number of arguments
fn same_size(text: &String, args: &Vec<String>) -> bool {
    let split_text_iter = text.split("{}").count();
    let args_iter = args.len();

    split_text_iter == args_iter + 1
}