#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::{interpreter::symbol_table::SymbolTable, lexer::tokens::{Keyword, TokenPosition}};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::{Token, TokenType}, errors::{Error, ErrorType}};

/// Binary operation node
#[derive(Debug, Clone)]
pub struct BinOpNode {
    pub left_node: Node,
    pub op_token: Token,
    pub right_node: Node,
}

impl BinOpNode {
    pub fn new(left_node: Node, op_token: Token, right_node: Node) -> BinOpNode {
        BinOpNode {
            left_node,
            op_token,
            right_node,
        }
    }
}

impl NodeVisit for BinOpNode {
    fn visit(&self, symbol_table: &mut SymbolTable) -> Result<Symbol, Error> {
        let left_symbol = self.left_node.visit(symbol_table)?;
        let right_symbol = self.right_node.visit(symbol_table)?;
        let op_token = self.op_token.clone();
        let result = match op_token.value {
            TokenType::Plus => left_symbol.add(&right_symbol)?,
            TokenType::Minus => left_symbol.sub(&right_symbol)?,
            TokenType::Star => left_symbol.mul(&right_symbol)?,
            TokenType::Slash => left_symbol.div(&right_symbol)?,
            TokenType::Caret => left_symbol.power(&right_symbol)?,
            TokenType::Less => left_symbol.lt(&right_symbol)?,
            TokenType::LessEqual => left_symbol.le(&right_symbol)?,
            TokenType::Greater => left_symbol.gt(&right_symbol)?,
            TokenType::GreaterEqual => left_symbol.ge(&right_symbol)?,
            TokenType::EqualEqual => left_symbol.eq(&right_symbol)?,
            TokenType::BangEqual => left_symbol.ne(&right_symbol)?,
            TokenType::Keyword(Keyword::And) => left_symbol.and(&right_symbol)?,
            TokenType::Keyword(Keyword::Or) => left_symbol.or(&right_symbol)?,
            _ => {
                return Err(Error::new_runtime(
                    ErrorType::InvalidOperation,
                    format!("Invalid operation: {:?}", op_token),
                    &self.get_position(),
                ));
            }
        };
        Ok(result)
    }

    fn get_position(&self) -> TokenPosition {
        self.op_token.position
    }
}

impl ToString for BinOpNode {
    fn to_string(&self) -> String {
        format!("({} {} {})", self.left_node.to_string(), self.op_token.to_string(), self.right_node.to_string())
    }
}