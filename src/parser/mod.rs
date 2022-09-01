//! Parses a token stream and outputs an abstract syntax tree.

pub mod nodes;

use nodes::{
    Node, 
    NumberNode,
    StringNode,
    BinOpNode, 
    UnaryOpNode,
    VarAssignmentNode,
    VarArithmeticAssignmentNode,
    VarAccessNode,
    IfExprNode,
    FuncDefNode,
    FuncCallNode,
    ListExprNode,
    StatementsNode,
    ReturnNode,
    AssertNode,
};

use crate::lexer::tokens::{Token, TokenType, Keyword};
use crate::errors::{Error};

/// Parses the tokens into an AST
pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
    current_token: Option<Token>,
}

type GrammarOutput = Result<Node, Error>;

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

    pub fn parse(&mut self) -> GrammarOutput {
        let res = self.gr_statements();

        if match self.get_current_token() {
            Some(token) => token.value != TokenType::EOF,
            None => true,
        } {
            return Err(Error::new_parser(
                format!("Unexpected token: {:?}, expected EOF", self.get_current_token().unwrap().value),
                &self.get_current_token_err()?.position,
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
        // DEBUG: println!("\nAdvanced to: \t\t\t\t{:?}\n", self.current_token);
        self.get_current_token()
    }

    /// Safely get the current token
    fn get_current_token(&self) -> Option<Token> {
        self.current_token.clone()
    }

    /// Get the current token and return an error if needed
    fn get_current_token_err(&self) -> Result<Token, Error> {
        match self.get_current_token() {
            Some(token) => Ok(token),
            None => Err(self.error_missing_token()),
        }
    }

    /// Peek at the next token in the stream
    #[allow(dead_code)]
    fn peek_token(&self) -> Result<Token, Error> {
        match Parser::try_get_token(&self.tokens, self.token_index + 1) {
            Some(token) => Ok(token),
            None => Err(self.error_missing_token()),
        }
    }

    /// Get the last token
    fn get_last_token(&self) -> Token {
        Parser::try_get_token(&self.tokens, self.token_index - 1).expect("Failed to get last token")
    }

    /// Check whether the end of file has been reached
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
        Error::new_parser(
            "Expected token".to_string(),
            &self.get_last_token().position,
        )
    }

    /// Expect a token of a given type at the current location.
    /// DOES NOT ADVANCE THE PARSER
    fn expect(&self, token_type: TokenType) -> Result<(), Error> {
        // DEBUG: println!("Expecting {:?} at {:?} ({:?})", token_type, self.get_current_token_err()?.position, self.get_current_token_err()?.value);
        match self.get_current_token() {
            Some(token) => {
                if token.value == token_type {
                    Ok(())
                } else {
                    Err(Error::new_parser(
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

    /// Statements
    fn gr_statements(&mut self) -> GrammarOutput {
        // DEBUG: println!("Statements\t\t\t\t{:?}", self.get_current_token());
        let mut statements: Vec<Node> = Vec::new();

        while let Some(current_tok) = self.get_current_token() {
            if [TokenType::EOF, TokenType::RightBrace, TokenType::RightParen, TokenType::RightSquare].contains(&current_tok.value) {
                break;
            } else {
                // DEBUG: println!("Getting statement from expression {:?}", self.get_current_token());
                statements.push(self.gr_statement()?);

            }
        };

        // DEBUG: println!("Statements completed, returning {:?}", statements);
        // DEBUG: println!("Current token: {:?}", self.get_current_token());

        Ok(Node::StatementsNode(Box::new(StatementsNode::new(statements))))
    }

    fn gr_statement(&mut self) -> GrammarOutput {
        // DEBUG: println!("Statement\t\t\t\t{:?}", self.get_current_token());
        let current_tok = self.get_current_token_err()?;
        let statement = match current_tok.value {
            TokenType::Keyword(Keyword::Return) => {
                // expecting return statement
                // DEBUG: println!("Expecting return statement");
                self.advance();
                
                // next token may be semicolon or expression
                let expr = match self.get_current_token_err()? {
                    Token {
                        value: TokenType::Semicolon,
                        ..
                    } => {
                        // return statement with no expression
                        // DEBUG: println!("Return statement with no expression");
                        self.advance();
                        None
                    }
                    _ => {
                        // return statement with expression
                        // DEBUG: println!("Return statement with expression");
                        Some(self.gr_expr()?)
                    }
                };
                
                Ok(Node::ReturnNode(Box::new(ReturnNode::new(expr))))
            }
            _ => {
                self.gr_expr()
            }
        };

        self.expect(TokenType::Semicolon)?;
        self.advance();
        statement
    }

    /// Expression
    fn gr_expr(&mut self) -> GrammarOutput {
        // DEBUG: println!("Expression\t\t\t\t{:?}", self.get_current_token());
        // check if keyword var instead
        let current_token = self.get_current_token().expect("Expected token");
        match current_token.value {
            TokenType::Keyword(Keyword::Let) => {
                // expecting variable assignment
                // DEBUG: println!("Expecting variable assignment");
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
                                return Err(Error::new_parser(
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

                // DEBUG: println!("Got identifier: {:?}", var_name_identifier);

                // ensure the next token is an equals sign
                match self.get_current_token_err()?.value {
                    TokenType::Equal => {
                        // Variable assignment
                        self.advance();

                        // DEBUG: println!("Got equals sign");

                        // ensure the next token is an expression
                        let expr = self.gr_expr()?;

                        // DEBUG: println!("Got expression");
                        Ok(Node::VarAssignmentNode(Box::new(VarAssignmentNode::new(var_name_identifier, expr))))
                    }

                    TokenType::PlusEqual | TokenType::MinusEqual |
                    TokenType::StarEqual | TokenType::SlashEqual => {
                        // Variable assignment with operator
                        let op_token = self.get_current_token_err()?;
                        self.advance();

                        let expr = self.gr_expr()?;
                        Ok(Node::VarArithmeticAssignmentNode(Box::new(VarArithmeticAssignmentNode::new(var_name_identifier, op_token, expr))))
                    },
                    _ => Err(Error::new_parser(
                        format!("Expected any equals sign, found {:?}", self.get_current_token_err()?.value),
                        &self.get_current_token_err()?.position,
                    ))
                }
                
            },

            TokenType::Keyword(Keyword::Assert) => {
                // expecting assert statement
                self.advance();

                let expr = self.gr_expr()?;
                Ok(Node::AssertNode(Box::new(AssertNode::new(expr))))
            }

            _ => {
                let mut left_node = self.gr_compare_expr()?;

                while !self.reached_eof() {
                    if [TokenType::Keyword(Keyword::And), TokenType::Keyword(Keyword::Or)].contains(&self.get_current_token_err()?.value) {
                        let op_token = self.get_current_token_err()?;
                        self.advance();
                        let right_node = self.gr_compare_expr()?;
                        left_node = Node::BinOpNode(Box::new(BinOpNode::new(left_node, op_token, right_node)));
                    } else {
                        break;
                    }
                }

                Ok(left_node)
            }
        }
    }

    /// Compare Expression
    fn gr_compare_expr(&mut self) -> GrammarOutput {
        // DEBUG: println!("Compare expression\t\t\t{:?}", self.get_current_token());
        
        let current_token = self.get_current_token().expect("No token found");

        match current_token.value {
            TokenType::Keyword(Keyword::Not) => {
                // negation
                self.advance();
                // expect another compare expression
                let right_node = self.gr_compare_expr()?;
                Ok(Node::UnaryOpNode(Box::new(UnaryOpNode::new(current_token, right_node))))
            },
            _ => {
                // find terms separated by operators
                let mut left_node = self.gr_arithmetic_expr()?;

                while !self.reached_eof() {
                    if [TokenType::EqualEqual, TokenType::BangEqual, TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual].contains(&self.get_current_token_err()?.value) {
                        let op_token = self.get_current_token_err()?;
                        self.advance();
                        let right_node = self.gr_arithmetic_expr()?;
                        left_node = Node::BinOpNode(Box::new(BinOpNode::new(left_node, op_token, right_node)));
                    } else {
                        break;
                    }
                }
                Ok(left_node)
            }
        }

        
    }

    /// Arithmetic Expression
    fn gr_arithmetic_expr(&mut self) -> GrammarOutput {
        // DEBUG: println!("Arith expression\t\t\t{:?}", self.get_current_token());
        // find terms separated by operators
        let mut left_node = self.gr_term()?;

        while !self.reached_eof() {
            if [TokenType::Plus, TokenType::Minus].contains(&self.get_current_token_err()?.value) {
                let op_token = self.get_current_token_err()?;
                self.advance();
                let right_node = self.gr_term()?;
                left_node = Node::BinOpNode(Box::new(BinOpNode::new(left_node, op_token, right_node)));
            } else {
                break;
            }
        }

        Ok(left_node)
    }

    /// Term
    fn gr_term(&mut self) -> GrammarOutput {
        // DEBUG: println!("Term\t\t\t\t\t{:?}", self.get_current_token());
        // find factors separated by operators

        let mut left_node = self.gr_factor()?;

        while !self.reached_eof() {
            if [TokenType::Star, TokenType::Slash].contains(&self.get_current_token_err()?.value) {
                let op_token = self.get_current_token_err()?;
                self.advance();
                let right_node = self.gr_factor()?;
                left_node = Node::BinOpNode(Box::new(BinOpNode::new(left_node, op_token, right_node)));
            } else {
                break;
            }
        }

        Ok(left_node)
    }

    /// Factor
    fn gr_factor(&mut self) -> GrammarOutput {
        // get atom
        // DEBUG: println!("Factor\t\t\t\t\t{:?}", self.get_current_token());
        let mut left_node = self.gr_call()?;

        while !self.reached_eof() {
            if self.get_current_token_err()?.value == TokenType::Caret {
                let op_token = self.get_current_token_err()?;
                self.advance();
                let right_node = self.gr_factor()?;
                left_node = Node::BinOpNode(Box::new(BinOpNode::new(left_node, op_token, right_node)));
            } else {
                break;
            }
        }

        Ok(left_node)
    }

    /// Call
    fn gr_call(&mut self) -> GrammarOutput {
        // DEBUG: println!("Call\t\t\t\t\t{:?}", self.get_current_token());
        let left_node = self.gr_atom()?;

        let current_tok = self.get_current_token_err()?;

        if current_tok.value == TokenType::LeftParen {
            // they are calling the identifier left_node
            self.advance();

            let mut args = Vec::new();
            while self.get_current_token_err()?.value != TokenType::RightParen {
                args.push(self.gr_expr()?);
                
                // if there is a comma, move on to the next argument
                if self.get_current_token_err()?.value == TokenType::Comma {
                    self.advance();
                    continue;
                }
            };

            self.expect(TokenType::RightParen)?;
            self.advance();

            return Ok(Node::FuncCallNode(Box::new(FuncCallNode::new(left_node, args))));
        }
        
        Ok(left_node)
    }

    /// Atom
    fn gr_atom(&mut self) -> GrammarOutput {
        // DEBUG: println!("Atom\t\t\t\t\t{:?}", self.get_current_token());
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
                        Ok(Node::NumberNode(Box::new(number_node)))
                    },

                    TokenType::String(_) => {
                        let string_node = StringNode::new(token);
                        self.advance();
                        Ok(Node::StringNode(Box::new(string_node)))
                    },

                    // if it is a unary operator (negation or positive), return a UnaryOpNode
                    TokenType::Plus | TokenType::Minus => {
                        let unary_op = self.get_current_token_err()?;
                        self.advance();
                        let factor = self.gr_atom()?;
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
                                    Err(Error::new_parser(
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

                    TokenType::LeftSquare => {
                        self.advance();
                        self.gr_list_expr()
                    }

                    // if it is an identifier, return a variable access node
                    TokenType::Identifier(_) => {
                        let var_access_node = VarAccessNode::new(token);
                        self.advance();
                        Ok(Node::VarAccessNode(Box::new(var_access_node)))
                    },

                    // If keyword
                    TokenType::Keyword(Keyword::If) => {
                        self.advance();
                        self.gr_if_expr()
                    },

                    // Func keyword
                    TokenType::Keyword(Keyword::Func) => {
                        self.advance();
                        self.gr_func_def()
                    }

                    // if no matches, return an error
                    _ => Err(Error::new_parser(
                        format!("Expected atom, found {:?}", token.value),
                        &token.position,
                    )),
                }
            },

            None => return Err(self.error_missing_token()),
        }
    }

    fn gr_list_expr(&mut self) -> GrammarOutput {
        // DEBUG: println!("List\t\t\t\t\t{:?}", self.get_current_token());
        let mut elements = Vec::new();

        while self.get_current_token_err()?.value != TokenType::RightSquare {
            elements.push(self.gr_expr()?);

            if self.get_current_token_err()?.value == TokenType::Comma {
                self.advance();
                continue;
            }
        }

        self.expect(TokenType::RightSquare)?;
        self.advance();

        Ok(Node::ListExprNode(Box::new(ListExprNode::new(elements))))
    }

    /// If Expression
    /// Must have advanced past 'if' keyword
    fn gr_if_expr(&mut self) -> GrammarOutput {
        // DEBUG: println!("If expression\t\t\t\t{:?}", self.get_current_token());

        // cases to consider
        let mut else_case: Option<Node> = None;

        // get condition expression
        let condition = self.gr_expr()?;

        // expect curly braces
        self.expect(TokenType::LeftBrace)?;
        self.advance();

        // get expression to be evaluated if condition is true
        let if_true = self.gr_expr()?;

        // expect curly braces
        self.expect(TokenType::RightBrace)?;
        self.advance();

        // optional else keyword
        match self.get_current_token() {
            Some(token) => {
                match token.value {
                    TokenType::Keyword(Keyword::Else) => {
                        self.advance();

                        // DEBUG: println!("Found else keyword, advanced to {:?}", self.get_current_token());

                        // nested if expression
                        if self.get_current_token_err()?.value == TokenType::Keyword(Keyword::If) {
                            self.advance();
                            // DEBUG: println!("Found nested if expression");
                            else_case = Some(self.gr_if_expr()?);
                            // DEBUG: println!("else_case: {:?}", else_case);
                        } else {
                            // DEBUG: println!("Found else expression");
                            // expect curly braces
                            self.expect(TokenType::LeftBrace)?;
                            self.advance();

                            // get expression to be evaluated if condition is false
                            let if_false = self.gr_expr()?;

                            // expect curly braces
                            self.expect(TokenType::RightBrace)?;
                            self.advance();

                            else_case = Some(if_false);
                        }
                    },
                    _ => {},
                }
            },
            None => {}
        }

        Ok(Node::IfExprNode(Box::new(IfExprNode::new(condition, if_true, else_case))))

    }

    /// Function Definition
    fn gr_func_def(&mut self) -> GrammarOutput {
        // DEBUG: println!("Function definition\t\t\t{:?}", self.get_current_token());
        // expect an identifier
        let current_tok = self.get_current_token_err()?;
        let identifier = match current_tok.value {
            TokenType::Identifier(_) => current_tok,
            _ => return Err(Error::new_parser(
                format!("Expected identifier, got {:?}", self.get_current_token_err()?.value),
                &self.get_current_token_err()?.position,
            )),
        };

        self.advance();

        // expect () or (identifier) or (identifier, identifier, ...)
        self.expect(TokenType::LeftParen)?;
        self.advance();

        let mut parameters: Vec<Token> = Vec::new();
        
        let current_tok = self.get_current_token_err()?;

        match current_tok.value {
            TokenType::Identifier(_) => {
                parameters.push(current_tok);
                self.advance();
            },
            _ => {}
        }

        // one or more ", <identifier>"
        while self.get_current_token_err()?.value == TokenType::Comma {
            // DEBUG: println!("Expecting identifier");
            self.advance();
            let current_tok = self.get_current_token_err()?;

            match current_tok.value {
                TokenType::Identifier(_) => {
                    parameters.push(current_tok);
                    self.advance();
                },
                _ => {
                    return Err(Error::new_parser(
                        format!("Expected identifier, got {:?}", self.get_current_token_err()?.value),
                        &self.get_current_token_err()?.position,
                    ))
                }
            }
        };

        // expect right parenthesis
        self.expect(TokenType::RightParen)?;
        self.advance();

        // expect { expr }
        self.expect(TokenType::LeftBrace)?;
        self.advance();

        // check if the next char is a right brace, if not expect an expression before
        let body: Option<Node> = if self.get_current_token_err()?.value == TokenType::RightBrace {
            // DEBUG: println!("Expecting empty expressionn");
            None
        } else {
            // DEBUG: println!("Expecting statements");
            Some(self.gr_statements()?)
        };

        // DEBUG: println!("Body for function: {:?}", body);

        self.expect(TokenType::RightBrace)?;
        self.advance();

        Ok(Node::FuncDefNode(Box::new(FuncDefNode::new(identifier, parameters, body))))
    }
}