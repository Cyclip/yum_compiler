pub mod lexer;
pub mod errors;
pub mod parser;

use std::env;

fn compile(source: &str) -> Result<(), errors::Error> {
    let mut lexer = lexer::Lexer::new(source.to_string());
    let tokens = lexer.make_tokens()?;
    println!("Tokens: {:#?}", tokens);
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    println!("{:#?}", ast);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();
    
    let expression = args.join(" ");

    match compile(&expression) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
}
