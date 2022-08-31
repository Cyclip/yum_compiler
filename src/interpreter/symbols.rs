use crate::parser::nodes::Node;

/// All the symbols that can be stored in the symbol table
#[derive(Debug, Clone)]
pub enum Symbol {
    Integer(i32),
    Float(f32),
    String(String),
    Node(Node),
    Function(FunctionSymbol),
    None,
}

pub enum SymbolType {
    Integer,
    Float,
    String,
    Node,
    Function,
    None,
}

#[derive(Debug, Clone)]
pub struct FunctionSymbol {
    pub name: String,
    pub args: Vec<String>,
    pub node: Node,
}

impl Symbol {
    pub fn get_type(&self) -> SymbolType {
        match self {
            Symbol::Integer(_) => SymbolType::Integer,
            Symbol::Float(_) => SymbolType::Float,
            Symbol::String(_) => SymbolType::String,
            Symbol::Node(_) => SymbolType::Node,
            Symbol::Function(_) => SymbolType::Function,
            Symbol::None => SymbolType::None,
        }
    }
}