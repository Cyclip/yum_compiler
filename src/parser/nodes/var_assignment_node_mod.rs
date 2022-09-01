#[allow(unused_imports)]
use super::{Node, NodeVisit, get_name_as_string};
use crate::interpreter::symbols::SymbolType;
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Variable assignment node
#[derive(Debug, Clone)]
pub struct VarAssignmentNode {
    pub identifier: Token,
    pub value: Node,
}

impl VarAssignmentNode {
    pub fn new(identifier: Token, value: Node) -> VarAssignmentNode {
        VarAssignmentNode {
            identifier,
            value,
        }
    }
}

impl NodeVisit for VarAssignmentNode {
    fn get_position(&self) -> crate::lexer::tokens::TokenPosition {
        self.identifier.position
    }

    fn visit(&self, symbol_table: &mut crate::interpreter::symbol_table::SymbolTable) -> Result<Symbol, Error> {
        let identifier_string = get_name_as_string(self.identifier.clone())?;

        let value = self.value.visit(symbol_table)?;

        symbol_table.set(identifier_string, value);
        Ok(Symbol::new(SymbolType::None, self.get_position()))
    }
}

impl ToString for VarAssignmentNode {
    fn to_string(&self) -> String {
        format!("VarAssignmentNode: {:?}", self.identifier.value)
    }
}