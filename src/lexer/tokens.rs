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
    LeftSquare, RightSquare,                            // [ ]
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,    // , . - + ; / *
    Caret, Underscore,                                  // ^ _

    // One or two character tokens
    Bang, BangEqual,                                    // ! !=
    Equal, EqualEqual,                                  // = ==
    Greater, GreaterEqual,                              // > >=
    Less, LessEqual,                                    // < <=
    // Arithmetic assignment tokens
    PlusEqual, MinusEqual, StarEqual, SlashEqual,       // += -= *= /=

    // Literals
    Integer(i32),                                       // 12345
    Float(f32),                                         // 123.45
    String(String),                                     // "hello"

    // Identifiers and keywords
    Identifier(String),                                 // my_identifier
    Keyword(Keyword),                                   // Any keyword

    // End of File
    EOF
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    And,
    Not,
    Or,
    If,
    Else,
    Elif,
    Func,
    Return,
    Assert,
}

impl Keyword {
    pub fn from(s: &str) -> Option<Keyword> {
        match s {
            "let" => Some(Keyword::Let),
            "and" => Some(Keyword::And),
            "not" => Some(Keyword::Not),
            "or" => Some(Keyword::Or),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "elif" => Some(Keyword::Elif),
            "func" => Some(Keyword::Func),
            "return" => Some(Keyword::Return),
            "assert" => Some(Keyword::Assert),
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