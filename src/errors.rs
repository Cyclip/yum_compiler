use std::fmt::Display;

use crate::lexer::LexerPosition;
use crate::lexer::tokens::TokenPosition;

/// Represents an error
#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub error_message: String,
    pub position: TokenPosition,
}

#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    InvalidToken,
    ParserError,
}

impl Error {
    pub fn new(error_type: ErrorType, error_message: String, position: &LexerPosition) -> Error {
        Error {
            error_type,
            error_message,
            position: TokenPosition::from(position),
        }
    }

    pub fn new_parser_error(error_message: String, position: &TokenPosition) -> Error {
        Error {
            error_type: ErrorType::ParserError,
            error_message,
            position: position.clone(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {} at line {}, column {}", self.error_type, self.error_message, self.position.line, self.position.column)
    }
}