#[allow(unused_imports)]
use super::{Node, NodeVisit, get_name_as_string};
use crate::{lexer::tokens::TokenType, interpreter::symbols::SymbolType};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Variable arithmetic assignment node
#[derive(Debug, Clone)]
pub struct VarArithmeticAssignmentNode {
    pub identifier: Token,
    pub op_token: Token,
    pub value: Node,
}

impl VarArithmeticAssignmentNode {
    pub fn new(identifier: Token, op_token: Token, value: Node) -> VarArithmeticAssignmentNode {
        VarArithmeticAssignmentNode {
            identifier,
            op_token,
            value,
        }
    }
}

impl NodeVisit for VarArithmeticAssignmentNode {
    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.op_token.position
    }

    fn visit(&self, symbol_table: crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        let right_symbol = self.value.visit(symbol_table)?;
        
        // get the current value of the variable
        let identifier_string = get_name_as_string(self.identifier)?;
        let left_symbol = match symbol_table.get(identifier_string) {
            Some(symbol) => symbol.clone(),
            None => return Err(Error::new_runtime(
                ErrorType::UndefinedVariable, 
                format!("Undefined variable '{:?}'", self.identifier.value), 
                &self.identifier.position
            )),
        };

        // perform the operation and assign
        symbol_table.set(identifier_string, left_symbol.add(&right_symbol)?);
        Ok(Symbol::new(SymbolType::None, self.get_position()))
    }
}