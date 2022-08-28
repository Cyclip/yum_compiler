pub mod lexer;
pub mod errors;
pub mod parser;

use std::io::Write;

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
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        match compile(&input) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
}
