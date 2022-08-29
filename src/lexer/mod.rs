//! Lexes and tokenizes a string into a stream of tokens.

pub mod tokens;

use tokens::{
    Token,
    TokenType,
    Keyword,
};

use crate::errors::{Error, ErrorType};

use self::tokens::TokenPosition;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const LETTERS: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];


#[derive(Debug)]
pub struct Lexer {
    // text to process
    text: String,
    position: LexerPosition,
}

/// Represents a position within a lexer
#[derive(Debug, Clone)]
pub struct LexerPosition {
    pub line: u32,
    pub column: u32,
    pub pos: usize,
    pub current_char: Option<char>,
    text: String,
}

impl LexerPosition {
    pub fn new(text: &String) -> LexerPosition {
        let current_char = text.chars().nth(0);
        LexerPosition {
            line: 1,
            column: 1,
            pos: 0,
            current_char,
            text: text.clone(),
        }
    }

    pub fn from(token_pos: TokenPosition) -> LexerPosition {
        LexerPosition {
            line: token_pos.line,
            column: token_pos.column,
            pos: 0,
            current_char: None,
            text: "".to_string(),
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.text.chars().nth(self.pos);

        if self.current_char == Some('\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let position = LexerPosition::new(&text);
        Lexer {
            text,
            position,
        }
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(current_char) = self.position.current_char {
            // ignore whitespace
            if current_char.is_whitespace() {
                self.position.advance();
                continue;
            }

            println!("Current char is {}", current_char);

            // check for single-character tokens
            match current_char {
                '(' => {
                    tokens.push(Token::new(TokenType::LeftParen, &self.position));
                    self.position.advance();
                    continue;
                },
                ')' => {
                    tokens.push(Token::new(TokenType::RightParen, &self.position));
                    self.position.advance();
                    continue;
                },
                '{' => {
                    tokens.push(Token::new(TokenType::LeftBrace, &self.position));
                    self.position.advance();
                    continue;
                },
                '}' => {
                    tokens.push(Token::new(TokenType::RightBrace, &self.position));
                    self.position.advance();
                    continue;
                },
                ',' => {
                    tokens.push(Token::new(TokenType::Comma, &self.position));
                    self.position.advance();
                    continue;
                },
                '.' => {
                    tokens.push(Token::new(TokenType::Dot, &self.position));
                    self.position.advance();
                    continue;
                },
                '-' => {
                    tokens.push(Token::new(TokenType::Minus, &self.position));
                    self.position.advance();
                    continue;
                },
                '+' => {
                    tokens.push(Token::new(TokenType::Plus, &self.position));
                    self.position.advance();
                    continue;
                },
                ';' => {
                    tokens.push(Token::new(TokenType::Semicolon, &self.position));
                    self.position.advance();
                    continue;
                },
                '*' => {
                    tokens.push(Token::new(TokenType::Star, &self.position));
                    self.position.advance();
                    continue;
                },
                '/' => {
                    tokens.push(Token::new(TokenType::Slash, &self.position));
                    self.position.advance();
                    continue;
                },
                '=' => {
                    let token = self.make_equals()?;
                    tokens.push(token);
                    continue;
                },
                '!' => {
                    let token = self.make_not_equals()?;
                    tokens.push(token);
                    continue;
                },
                '>' => {
                    let token = self.make_greater_than()?;
                    tokens.push(token);
                    continue;
                },
                '<' => {
                    let token = self.make_less_than()?;
                    tokens.push(token);
                    continue;
                },
                '^' => {
                    tokens.push(Token::new(TokenType::Caret, &self.position));
                    self.position.advance();
                    continue;
                },
                '"' => {
                    let token = self.make_string()?;
                    tokens.push(token);
                    continue;
                },
                '[' => {
                    tokens.push(Token::new(TokenType::LeftSquare, &self.position));
                    self.position.advance();
                    continue;
                },
                ']' => {
                    tokens.push(Token::new(TokenType::RightSquare, &self.position));
                    self.position.advance();
                    continue;
                },
                '_' => {
                    tokens.push(Token::new(TokenType::Underscore, &self.position));
                    self.position.advance();
                    continue;
                }
                _ => (),
            };
            
            // check if number
            if self.is_digit(current_char) {
                let number_token = self.make_number()?;
                tokens.push(number_token);
                continue;
            }

            // check if identifier
            if self.is_letter(current_char) {
                let identifier_token = self.make_identifier()?;
                tokens.push(identifier_token);
                continue;
            }

            // invalid token
            return Err(Error::new(
                ErrorType::InvalidToken,
                format!("Unexpected character: '{}'", current_char),
                &self.position
            ));
        }

        tokens.push(Token::new(TokenType::EOF, &self.position));

        Ok(tokens)
    }

    fn get_current_char<S: ToString>(&self, error: S) -> Result<char, Error> {
        match self.position.current_char {
            Some(c) => Ok(c),
            None => Err(Error::new(
                ErrorType::InvalidToken,
                error.to_string(),
                &self.position
            ))
        }
    }

    fn expect(&self, expected: char) -> Result<(), Error> {
        match self.get_current_char(format!("Expected '{}'", expected)) {
            Ok(c) => {
                if c == expected {
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorType::InvalidToken,
                        format!("Expected '{}', got {}", expected, c),
                        &self.position
                    ))
                }
            }
            Err(e) => Err(e)
        }
    }

    /// Try to make a string
    fn make_string(&mut self) -> Result<Token, Error> {
        let mut string = String::new();

        self.expect('"')?;
        self.position.advance();

        while self.get_current_char("Unterminated string")? != '"' {
            string.push(self.get_current_char("Unterminated string")?);
            self.position.advance();
        };

        self.expect('"')?;
        self.position.advance();

        Ok(Token::new(TokenType::String(string), &self.position))
    }

    /// Peek at the next character in the lexer
    fn peek_char(&self) -> Option<char> {
        self.text.chars().nth(self.position.pos + 1)
    }

    /// Try to make >=, if not just return !
    fn make_less_than(&mut self) -> Result<Token, Error> {
        let token_type = if self.peek_char() == Some('=') {
            self.position.advance();
            TokenType::LessEqual
        } else {
            TokenType::Less
        };

        let token = Token::new(token_type, &self.position);

        self.position.advance();

        Ok(token)
    }

    /// Try to make >=, if not just return !
    fn make_greater_than(&mut self) -> Result<Token, Error> {
        let token_type = if self.peek_char() == Some('=') {
            self.position.advance();
            TokenType::GreaterEqual
        } else {
            TokenType::Greater
        };

        let token = Token::new(token_type, &self.position);

        self.position.advance();

        Ok(token)
    }

    /// Try to make !=, if not just return !
    fn make_not_equals(&mut self) -> Result<Token, Error> {
        let token_type = if self.peek_char() == Some('=') {
            self.position.advance();
            TokenType::BangEqual
        } else {
            TokenType::Bang
        };

        let token = Token::new(token_type, &self.position);

        self.position.advance();

        Ok(token)
    }

    /// Try to make an == operator, if not just return a single =
    fn make_equals(&mut self) -> Result<Token, Error> {
        let token_type = if self.peek_char() == Some('=') {
            self.position.advance();
            TokenType::EqualEqual
        } else {
            TokenType::Equal
        };

        let token = Token::new(token_type, &self.position);

        self.position.advance();

        Ok(token)
    }

    /// Create an identifier token from the current position
    fn make_identifier(&mut self) -> Result<Token, Error> {
        let mut identifier: String = String::new();

        while let Some(current_char) = self.position.current_char {
            if current_char.is_whitespace() {
                // identifier is finished
                break;
            }

            if self.is_letter(current_char) || current_char == '_' {
                identifier.push(current_char);
                self.position.advance();
                continue;
            } else if self.is_digit(current_char) {
                // ensure it is not the first character
                if identifier.is_empty() {
                    return Err(Error::new(
                        ErrorType::InvalidToken,
                        format!("Unexpected character: '{}'", current_char),
                        &self.position
                    ));
                } else {
                    identifier.push(current_char);
                    self.position.advance();
                    continue;
                }
            } else {
                // return Err(Error::new(
                //     ErrorType::InvalidToken,
                //     format!("Unexpected character: '{}'", current_char),
                //     &self.position
                // ));
                break;
            }
        };

        // determine whether it is a identifier or keyword

        match Keyword::from(&identifier) {
            Some(keyword) => Ok(Token::new(TokenType::Keyword(keyword), &self.position)),
            None => Ok(Token::new(TokenType::Identifier(identifier), &self.position)),
        }

    }

    /// Create a number token from the current position
    fn make_number(&mut self) -> Result<Token, Error> {
        let mut number = String::new();
        let mut dot_count = 0;

        while let Some(current_char) = self.position.current_char {
            if self.is_digit(current_char) {
                number.push(current_char);
                self.position.advance();
            } else if current_char == '.' {
                if dot_count > 0 {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        "Number cannot have more than one decimal point".to_string(),
                        &self.position
                    ));
                }
                dot_count += 1;
                number.push(current_char);
                self.position.advance();
            } else {
                break;
            }
        }

        if dot_count == 0 {
            Ok(Token::new(TokenType::Integer(number.parse::<i32>().unwrap()), &self.position))
        } else {
            Ok(Token::new(TokenType::Float(number.parse::<f32>().unwrap()), &self.position))
        }
    }

    fn is_digit(&self, c: char) -> bool {
        DIGITS.contains(&c)
    }

    fn is_letter(&self, c: char) -> bool {
        LETTERS.contains(&c)
    }
}