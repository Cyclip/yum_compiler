use crate::{parser::nodes::Node, lexer::tokens::TokenPosition};

/// All the symbols that can be stored in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub value: SymbolType,
    pub position: TokenPosition,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Integer(i32),
    Float(f32),
    String(String),
    Node(Node),
    Function(FunctionSymbol),
    None,
}

#[derive(Debug, Clone)]
pub struct FunctionSymbol {
    pub name: String,
    pub args: Vec<String>,
    pub node: Node,
}

impl Symbol {
    pub fn new(value: SymbolType, position: TokenPosition) -> Symbol {
        Symbol {
            value,
            position,
        }
    }
}