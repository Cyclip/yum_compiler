pub mod lexer;
pub mod errors;
pub mod parser;

fn main() {
    // Generate tokens from input
    let mut lexer = lexer::Lexer::new(String::from("3 * 3 * 5"));
    let tokens = lexer.make_tokens().unwrap();
    println!("Tokenized: {:#?}", tokens);

    // Generate AST from tokens
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse().unwrap();
    println!("AST: {:#?}", ast);
}
