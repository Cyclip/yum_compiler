//! Determines available tokens to be parsed

use crate::lexer::LexerPosition;

/// Represents a single token
#[derive(Debug, Clone)]
pub struct Token {
    pub r#value: TokenType,
    pub position: TokenPosition,
}

/// Position of a token
#[derive(Copy, Clone)]
pub struct TokenPosition {
    pub line: u32,
    pub column: u32,
}

/// Enum for differnt token types
#[derive(Debug, Clone, PartialEq)]
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
    Integer(i32),                                       // 12345
    Float(f32),                                         // 123.45

    // Identifiers and keywords
    Identifier(String),                                 // my_identifier
    Keyword(Keyword),                                   // Any keyword

    // End of File
    EOF
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
}

impl Keyword {
    pub fn from(s: &str) -> Option<Keyword> {
        match s {
            "let" => Some(Keyword::Let),
            _ => None,
        }
    }
}

impl Token {
    pub fn new(r#value: TokenType, position: &LexerPosition) -> Token {
        Token {
            r#value,
            position: TokenPosition::from(position),
        }
    }
}

impl TokenPosition {
    pub fn from(position: &LexerPosition) -> TokenPosition {
        TokenPosition {
            line: position.line,
            column: position.column,
        }
    }
}

impl std::fmt::Debug for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenPosition {{ line: {}, column: {} }}", self.line, self.column)
    }
}