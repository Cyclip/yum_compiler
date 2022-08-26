use std::fmt::Display;

/// Represents an error
#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub error_message: String,
    pub error_line: u32,
    pub error_column: u32,
}

#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    InvalidToken,
}

impl Error {
    pub fn new(error_type: ErrorType, error_message: String, position: usize, source: &String) -> Error {
        let (error_line, error_column) = Error::get_char_position(position, source);

        Error {
            error_type,
            error_message,
            error_line,
            error_column,
        }
    }

    fn get_char_position(position: usize, source: &String) -> (u32, u32) {
        let mut line = 1;
        let mut column = 0;

        for (i, c) in source.chars().enumerate() {
            column += 1;

            if c == '\n' {
                column = 0;
                line += 1;
            }

            if i == position {
                break
            }
        }

        (line, column)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {} at line {}, column {}", self.error_type, self.error_message, self.error_line, self.error_column)
    }
}