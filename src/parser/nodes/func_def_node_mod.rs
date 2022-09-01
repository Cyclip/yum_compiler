#[allow(unused_imports)]
use super::{Node, NodeVisit, get_name_as_string, string_parameters};
use crate::{lexer::tokens::TokenPosition, interpreter::symbols::SymbolType};
#[allow(unused_imports)]
use crate::{interpreter::symbols::{Symbol, FunctionSymbol}, lexer::tokens::Token, errors::{Error, ErrorType}};
use crate::interpreter::symbol_table::SymbolTable;

/// Function definition node
#[derive(Debug, Clone)]
pub struct FuncDefNode {
    pub identifier: Token,
    pub parameters: Vec<Token>,
    pub body: Option<Node>,
}

impl FuncDefNode {
    pub fn new(identifier: Token, parameters: Vec<Token>, body: Option<Node>) -> FuncDefNode {
        FuncDefNode {
            identifier,
            parameters,
            body,
        }
    }
}

impl NodeVisit for FuncDefNode {
    fn visit(&self, symbol_table: &mut SymbolTable) -> Result<Symbol, Error> {
        let identifier_string = get_name_as_string(self.identifier.clone())?;

        // set function in symbol table
        let func_symbol = SymbolType::Function(FunctionSymbol::new(identifier_string.clone(), string_parameters(self.parameters.clone()), self.body.clone().unwrap()));
        symbol_table.set(identifier_string, Symbol::new(func_symbol, self.identifier.position));
        Ok(Symbol::new(SymbolType::None, self.identifier.position))
    }
    
    fn get_position(&self) -> TokenPosition {
        self.identifier.position
    }
}
