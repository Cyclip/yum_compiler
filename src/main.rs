pub mod lexer;
pub mod errors;
pub mod parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    
    let expression = args.join(" ");

    // Generate tokens from input
    let mut lexer = lexer::Lexer::new(expression);
    let tokens = lexer.make_tokens().unwrap();
    println!("Tokenized: {:#?}", tokens);

    // Generate AST from tokens
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse().unwrap();
    println!("AST: {:#?}", ast);
}
