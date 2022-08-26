pub mod lexer;
pub mod errors;

fn main() {
    let mut lexer = lexer::Lexer::new(String::from("4.5 + (22 * 5);"));
    let tokens = lexer.make_tokens();
    println!("Tokenized: {:#?}", tokens);
}
