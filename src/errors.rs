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
    InvalidSyntax,
    AssertError,
    InvalidOperation,
    Exception,
    TypeError,
    UndefinedVariable,
    ArgumentError,
    IOError,
}

impl Error {
    /// New lexer error
    pub fn new_lexer(error_type: ErrorType, error_message: String, position: &LexerPosition) -> Error {
        Error {
            error_type,
            error_message,
            position: TokenPosition::from(position),
        }
    }

    /// New parser error
    pub fn new_parser(error_message: String, position: &TokenPosition) -> Error {
        Error {
            error_type: ErrorType::ParserError,
            error_message,
            position: position.clone(),
        }
    }

    /// New runtime error
    pub fn new_runtime(error_type: ErrorType, error_message: String, position: &TokenPosition) -> Error {
        Error {
            error_type,
            error_message,
            position: position.clone(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.position.is_internal() {
            write!(
                f, "[Internal error] {:?}: {}",
                self.error_type,
                self.error_message,
            )
        } else {
            write!(
                f, "{:?}: {} at line {}, column {}", 
                self.error_type, 
                self.error_message, 
                self.position.line, 
                self.position.column,
            )
        }
    }
}