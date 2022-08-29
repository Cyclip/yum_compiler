pub mod lexer;
pub mod errors;
pub mod parser;

use std::env;

use errors::Error;
use parser::nodes::Node;

fn compile(source: String) -> Result<Node, Error> {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.make_tokens()?;

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();
    ast
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: {} <filename>", env::args().nth(0).unwrap());
            return;
        }
    };

    let source_code = match std::fs::read_to_string(filename) {
        Ok(source_code) => source_code,
        Err(error) => {
            println!("Error while reading source code: {}", error);
            return;
        }
    };

    let ast = match compile(source_code) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("{:#?}", ast);
}