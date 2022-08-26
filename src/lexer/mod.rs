pub mod tokens;

use tokens::{
    Token, TokenType
};

use crate::errors::{Error, ErrorType};

const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

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

    pub fn advance(&mut self) {
        self.pos += 1;
        self.column += 1;
        self.current_char = self.text.chars().nth(self.pos);
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
            println!("Current char: '{}'", current_char);
            // ignore whitespace
            if current_char.is_whitespace() {
                self.position.advance();
                continue;
            }

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
                }
                _ => (),
            };
            
            // check if number
            if self.is_digit(current_char) {
                match self.make_number() {
                    Ok(token) => tokens.push(token),
                    Err(e) => return Err(e),
                };

                continue;
            }

            // invalid token
            return Err(Error::new(
                ErrorType::InvalidToken,
                format!("Unexpected character: '{}'", current_char),
                &self.position
            ));
        }

        Ok(tokens)
    }

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
            Ok(Token::new(TokenType::Integer(number.parse::<u32>().unwrap()), &self.position))
        } else {
            Ok(Token::new(TokenType::Float(number.parse::<f32>().unwrap()), &self.position))
        }
    }

    fn is_digit(&self, c: char) -> bool {
        DIGITS.contains(&c.to_string().as_str())
    }
}