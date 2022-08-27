//! Parses a token stream and outputs an abstract syntax tree.

pub mod nodes;

use nodes::{
    Node, 
    NumberNode, 
    BinOpNode, 
    UnaryOpNode,
    VarAssignmentNode,
    VarAccessNode,
};
use crate::lexer::tokens::{Token, TokenType, Keyword};
use crate::errors::{Error, ErrorType};

/// Parses the tokens into an AST
pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        match tokens.len() {
            0 => panic!("No tokens to parse"),
            _ => {
                let token_index = 0;
                let current_token = Parser::try_get_token(&tokens, 0);
                Parser {
                    tokens,
                    token_index,
                    current_token,
                }
            }
        }
    }

    pub fn parse(&mut self) -> Result<Node, Error> {
        let res = self.gr_expr();

        if match self.get_current_token() {
            Some(token) => token.value != TokenType::EOF,
            None => true,
        } {
            return Err(Error::new_parser_error(
                "Expected EOF".to_string(),
                &self.get_current_token().unwrap().position,
            ));
        } else {
            return res;
        }
    }

    /// Attempt to get a token at a given index
    /// Returns None if no tokens are found
    fn try_get_token(tokens: &Vec<Token>, pos: usize) -> Option<Token> {
        let token: Option<&Token> = tokens.get(pos);
        match token {
            Some(token) => {
                let token_cloned = token.clone();
                Some(token_cloned)
            }
            None => {
                None
            }
        }
    }

    /// Get the next token in the stream
    fn advance(&mut self) -> Option<Token> {
        self.token_index += 1;
        self.current_token = Parser::try_get_token(&self.tokens, self.token_index);
        self.get_current_token()
    }

    fn get_current_token(&self) -> Option<Token> {
        self.current_token.clone()
    }

    fn reached_eof(&self) -> bool {
        match self.current_token {
            Some(_) => false,
            None => true,
        }
    }

    /// Return a new error when the parser expects a
    /// token but does not find it. Reduces repeating
    /// code.
    fn error_missing_token(&self) -> Error {
        Error::new_parser_error(
            "Expected token".to_string(),
            &self.get_current_token().unwrap().position,
        )
    }

    /// Expect a token of a given type at the current location
    /// DOES NOT ADVANCE THE PARSER
    fn expect(&self, token_type: TokenType) -> Result<(), Error> {
        println!("Expecting {:?} at {:?} ({:?})", token_type, self.get_current_token().unwrap().position, self.get_current_token().unwrap().value);
        match self.get_current_token() {
            Some(token) => {
                if token.value == token_type {
                    Ok(())
                } else {
                    Err(Error::new_parser_error(
                        format!("Expected {:?}", token_type),
                        &token.position,
                    ))
                }
            }
            None => Err(self.error_missing_token()),
        }
    }

    // ================ Grammar ================
    // Located in src/grammar.txt
    // =========================================

    /// Factor
    fn gr_factor(&mut self) -> Result<Node, Error> {
        println!("gr_factor on {:?}", self.get_current_token());
        // try to find int/longint/float/double
        // if not found, raise error

        match self.get_current_token() {
            // ensure that there is a token at the current position
            Some(token) => {
                match token.value {
                    // if it is an int/longint/float/double, return a NumberNode
                    TokenType::Integer(_) | TokenType::Float(_) => {
                        let number_node = NumberNode::new(token);
                        self.advance();
                        Ok(Node::Number(Box::new(number_node)))
                    },

                    // if it is a unary operator (negation or positive), return a UnaryOpNode
                    TokenType::Plus | TokenType::Minus => {
                        let unary_op = self.get_current_token().unwrap();
                        self.advance();
                        let factor = self.gr_factor()?;
                        let unary_op_node = UnaryOpNode::new(unary_op, factor);
                        Ok(Node::UnaryOpNode(Box::new(unary_op_node)))
                    },

                    // if it is a left parenthesis, return a nested expression
                    TokenType::LeftParen => {
                        self.advance();
                        let expr = self.gr_expr()?;
                        match self.get_current_token() {
                            Some(token) => {
                                if token.value == TokenType::RightParen {
                                    self.advance();
                                    Ok(expr)
                                } else {
                                    Err(Error::new_parser_error(
                                        format!("Expected ')', got {:?}", token.value),
                                        &token.position,
                                    ))
                                }
                            }
                            None => {
                                return Err(self.error_missing_token())
                            }
                        }
                    },

                    TokenType::Identifier(_) => {
                        let var_access_node = VarAccessNode::new(token);
                        self.advance();
                        Ok(Node::VarAccessNode(Box::new(var_access_node)))
                    }

                    _ => Err(Error::new_parser_error(
                        format!("Expected factor, found {:?}", token.value),
                        &token.position,
                    )),
                }
            }
            None => return Err(self.error_missing_token()),
        }
    }

    /// Term
    fn gr_term(&mut self) -> Result<Node, Error> {
        println!("gr_term on {:?}", self.get_current_token());
        // find factors separated by operators

        let mut left_node = self.gr_factor()?;

        while !self.reached_eof() {
            if [TokenType::Star, TokenType::Slash].contains(&self.get_current_token().unwrap().value) {
                let op_token = self.get_current_token().unwrap();
                self.advance();
                let right_node = self.gr_factor()?;
                left_node = Node::BinOp(Box::new(BinOpNode::new(left_node, op_token, right_node)));
            } else {
                break;
            }
        }

        Ok(left_node)
    }

    /// Expression
    fn gr_expr(&mut self) -> Result<Node, Error> {
        println!("gr_expr on {:?}", self.get_current_token());
        // check if keyword var instead
        let current_token = self.get_current_token().expect("Expected token");
        match current_token.value {
            TokenType::Keyword(Keyword::Let) => {
                // expecting variable assignment
                println!("Expecting variable assignment");
                self.advance();

                // ensure the next token is a variable name (identifier)
                let var_name_identifier = match self.get_current_token() {
                    // check if a token exists at the current position
                    Some(token) => {
                        // token exists, check if its an identifier or not
                        match token.value {
                            // token is an identifier
                            // return it
                            TokenType::Identifier(_) => {
                                self.advance();
                                token
                            },
                            _ => {
                                // token is not an identifier
                                return Err(Error::new_parser_error(
                                    format!("Expected identifier, found {:?}", token.value),
                                    &token.position,
                                ));
                            }
                        }
                    },
                    None => {
                        return Err(self.error_missing_token())
                    }
                };

                println!("Got identifier: {:?}", var_name_identifier);

                // ensure the next token is an equals sign
                self.expect(TokenType::Equal)?;
                self.advance();

                println!("Got equals sign");

                // ensure the next token is an expression
                let expr = self.gr_expr()?;

                println!("Got expression");

                // ensure final token is a semicolon
                self.expect(TokenType::Semicolon)?;
                self.advance();

                println!("Got semicolon");
                Ok(Node::VarAssignmentNode(Box::new(VarAssignmentNode::new(var_name_identifier, expr))))
            },
            _ => {
                // find terms separated by operators
                let mut left_node = self.gr_term()?;

                while !self.reached_eof() {
                    if [TokenType::Plus, TokenType::Minus].contains(&self.get_current_token().unwrap().value) {
                        let op_token = self.get_current_token().unwrap();
                        self.advance();
                        let right_node = self.gr_term()?;
                        left_node = Node::BinOp(Box::new(BinOpNode::new(left_node, op_token, right_node)));
                    } else {
                        break;
                    }
                }

                Ok(left_node)
            }
        }
    }
}