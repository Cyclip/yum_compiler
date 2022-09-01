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

    pub fn internal() -> TokenPosition {
        TokenPosition {
            line: 0,
            column: 0,
        }
    }

    pub fn is_internal(&self) -> bool {
        self.line == 0 && self.column == 0
    }
}

impl std::fmt::Debug for TokenPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenPosition {{ line: {}, column: {} }}", self.line, self.column)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#value)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TokenType::LeftParen => "[".to_string(),
            TokenType::RightParen => "]".to_string(),
            TokenType::LeftBrace => "{".to_string(),
            TokenType::RightBrace => "}".to_string(),
            TokenType::LeftSquare => "(".to_string(),
            TokenType::RightSquare => ")".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Slash => "/".to_string(),
            TokenType::Star => "*".to_string(),
            TokenType::Caret => "^".to_string(),
            TokenType::Underscore => "_".to_string(),
            TokenType::Bang => "!".to_string(),
            TokenType::BangEqual => "!=".to_string(),
            TokenType::Equal => "=".to_string(),
            TokenType::EqualEqual => "==".to_string(),
            TokenType::Greater => ">".to_string(),
            TokenType::GreaterEqual => ">=".to_string(),
            TokenType::Less => "<".to_string(),
            TokenType::LessEqual => "<=".to_string(),
            TokenType::PlusEqual => "+=".to_string(),
            TokenType::MinusEqual => "-=".to_string(),
            TokenType::StarEqual => "*=".to_string(),
            TokenType::SlashEqual => "/=".to_string(),
            TokenType::Integer(i) => i.to_string(),
            TokenType::Float(f) => f.to_string(),
            TokenType::String(s) => s.to_string(),
            TokenType::Identifier(s) => s.to_string(),
            TokenType::Keyword(k) => k.to_string(),
            TokenType::EOF => "EOF".to_string(),
        };

        write!(f, "{}", text)
    }
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Let => "let".to_string(),
            Keyword::And => "and".to_string(),
            Keyword::Not => "not".to_string(),
            Keyword::Or => "or".to_string(),
            Keyword::If => "if".to_string(),
            Keyword::Else => "else".to_string(),
            Keyword::Elif => "elif".to_string(),
            Keyword::Func => "func".to_string(),
            Keyword::Return => "return".to_string(),
            Keyword::Assert => "assert".to_string(),
        }
    }
}