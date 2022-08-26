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
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let current_char = text.chars().nth(0);
        Lexer {
            text,
            pos: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.text.chars().nth(self.pos);
        println!("Advancing to pos: {}, char: {:?}", self.pos, self.current_char);
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(current_char) = self.current_char {
            println!("Current char: '{}'", current_char);
            // ignore whitespace
            if current_char.is_whitespace() {
                self.advance();
                continue;
            }

            // check for single-character tokens
            match current_char {
                '(' => {
                    tokens.push(Token::new(TokenType::LeftParen));
                    self.advance();
                    continue;
                },
                ')' => {
                    tokens.push(Token::new(TokenType::RightParen));
                    self.advance();
                    continue;
                },
                '{' => {
                    tokens.push(Token::new(TokenType::LeftBrace));
                    self.advance();
                    continue;
                },
                '}' => {
                    tokens.push(Token::new(TokenType::RightBrace));
                    self.advance();
                    continue;
                },
                ',' => {
                    tokens.push(Token::new(TokenType::Comma));
                    self.advance();
                    continue;
                },
                '.' => {
                    tokens.push(Token::new(TokenType::Dot));
                    self.advance();
                    continue;
                },
                '-' => {
                    tokens.push(Token::new(TokenType::Minus));
                    self.advance();
                    continue;
                },
                '+' => {
                    tokens.push(Token::new(TokenType::Plus));
                    self.advance();
                    continue;
                },
                ';' => {
                    tokens.push(Token::new(TokenType::Semicolon));
                    self.advance();
                    continue;
                },
                '*' => {
                    tokens.push(Token::new(TokenType::Star));
                    self.advance();
                    continue;
                },
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
                self.pos,
                &self.text,
            ));
        }

        Ok(tokens)
    }

    fn make_number(&mut self) -> Result<Token, Error> {
        let mut number = String::new();
        let mut dot_count = 0;

        while let Some(current_char) = self.current_char {
            if self.is_digit(current_char) {
                number.push(current_char);
                self.advance();
            } else if current_char == '.' {
                if dot_count > 0 {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        "Number cannot have more than one decimal point".to_string(),
                        self.pos,
                        &self.text,
                    ));
                }
                dot_count += 1;
                number.push(current_char);
                self.advance();
            } else {
                break;
            }
        }

        if dot_count == 0 {
            Ok(Token::new(TokenType::Integer(number.parse::<u32>().unwrap())))
        } else {
            Ok(Token::new(TokenType::Float(number.parse::<f32>().unwrap())))
        }
    }

    fn is_digit(&self, c: char) -> bool {
        DIGITS.contains(&c.to_string().as_str())
    }
}