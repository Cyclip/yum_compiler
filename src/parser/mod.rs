mod nodes;

use nodes::{Node, NumberNode, BinOpNode, UnaryOpNode};
use crate::lexer::tokens::{Token, TokenType};
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

        if match self.current_token {
            Some(token) => token.value != TokenType::EOF,
            None => true,
        } {
            return Err(Error::new_parser_error(
                "Expected EOF".to_string(),
                &self.current_token.unwrap().position,
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
        println!("Advancing to token: {:#?}", self.current_token);
        self.current_token
    }

    fn reached_eof(&self) -> bool {
        match self.current_token {
            Some(_) => false,
            None => true,
        }
    }

    // ================ Grammar ================
    // Located in src/grammar.txt
    // =========================================

    /// Factor
    fn gr_factor(&mut self) -> Result<Node, Error> {
        println!("Parsing factor on token: {:#?}", self.current_token);
        // try to find int/longint/float/double
        // if not found, raise error

        match self.current_token {
            // ensure that there is a token at the current position
            Some(token) => {
                match token.value {
                    // if it is an int/longint/float/double, return a NumberNode
                    TokenType::Integer(_) | TokenType::LongInteger(_) | TokenType::Float(_) | TokenType::Double(_) => {
                        let number_node = NumberNode::new(token);
                        self.advance();
                        Ok(Node::Number(Box::new(number_node)))
                    },
                    // if it is a unary operator (negation or positive), return a UnaryOpNode
                    TokenType::Plus | TokenType::Minus => {
                        let unary_op = self.current_token.unwrap();
                        self.advance();
                        let factor = self.gr_factor()?;
                        let unary_op_node = UnaryOpNode::new(unary_op, factor);
                        Ok(Node::UnaryOpNode(Box::new(unary_op_node)))
                    }
                    _ => Err(Error::new_parser_error(
                        format!("Expected number, found {:?}", token.value),
                        &token.position,
                    )),
                }
            }
            None => Err(Error::new_parser_error(
                format!("Expected number, found none"),
                &self.current_token.unwrap().position,
            )),
        }
    }

    /// Term
    fn gr_term(&mut self) -> Result<Node, Error> {
        println!("Parsing term on token: {:#?}", self.current_token);
        // find factors separated by operators

        let mut left_node = self.gr_factor()?;

        while !self.reached_eof() {
            if [TokenType::Star, TokenType::Slash].contains(&self.current_token.unwrap().value) {
                let op_token = self.current_token.unwrap();
                self.advance();
                let right_node = self.gr_factor()?;
                left_node = Node::BinOp(Box::new(BinOpNode::new(left_node, op_token, right_node)));
            } else {
                break;
            }
        }

        println!("Completed term: {:#?}", left_node);

        Ok(left_node)
    }

    /// Expression
    fn gr_expr(&mut self) -> Result<Node, Error> {
        println!("Parsing expr on token: {:#?}", self.current_token);
        // find terms separated by operators
        let mut left_node = self.gr_term()?;

        while !self.reached_eof() {
            if [TokenType::Plus, TokenType::Minus].contains(&self.current_token.unwrap().value) {
                let op_token = self.current_token.unwrap();
                self.advance();
                let right_node = self.gr_term()?;
                left_node = Node::BinOp(Box::new(BinOpNode::new(left_node, op_token, right_node)));
            } else {
                break;
            }
        }

        println!("Completed expr: {:#?}", left_node);

        Ok(left_node)
    }
}