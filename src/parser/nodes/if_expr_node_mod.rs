#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::interpreter::symbols::SymbolType;
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// If statement expression node
#[derive(Debug, Clone)]
pub struct IfExprNode {
    pub condition: Node,
    pub if_true: Node,
    pub if_false: Option<Node>,
}

impl IfExprNode {
    pub fn new(condition: Node, if_true: Node, if_false: Option<Node>) -> IfExprNode {
        IfExprNode {
            condition,
            if_true,
            if_false,
        }
    }
}

impl NodeVisit for IfExprNode {
    fn visit(&self, symbol_table: &mut crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        let condition_symbol = self.condition.visit(symbol_table)?;
        match condition_symbol.value {
            SymbolType::Integer(value) => {
                if value == 0 {
                    match self.if_false.clone() {
                        Some(if_false) => return if_false.visit(symbol_table),
                        None => return Ok(Symbol::new(SymbolType::None, self.get_position())),
                    }
                } else {
                    return self.if_true.visit(symbol_table);
                }
            }
            _ => return Err(Error::new_runtime(
                ErrorType::Exception,
                "If statement condition must be an integer".to_string(),
                &self.get_position(),
            )),
        }
    }

    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.condition.get_position()
    }
}

impl ToString for IfExprNode {
    fn to_string(&self) -> String {
        match self.if_false.clone() {
            Some(if_false) => format!(
                "if {} {{ {} }} else {{ {} }}",
                self.condition.to_string(),
                self.if_true.to_string(),
                if_false.to_string()
            ),
            None => format!(
                "if {} {{ {} }}",
                self.condition.to_string(),
                self.if_true.to_string()
            ),
        }
    }
}