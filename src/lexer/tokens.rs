/// Represents a single token
#[derive(Debug)]
pub struct Token {
    r#value: TokenType,
}

/// Enum for differnt token types
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,       // ( ) { }
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,    // , . - + ; / *

    // One or two character tokens
    Bang, BangEqual,                                    // ! !=
    Equal, EqualEqual,                                  // = ==
    Greater, GreaterEqual,                              // > >=
    Less, LessEqual,                                    // < <=

    // Number literals
    Integer(u32),
    LongInteger(u64),
    Float(f32),
    Double(f64),
}

impl Token {
    pub fn new(r#value: TokenType) -> Token {
        Token { r#value }
    }
}