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

impl FunctionSymbol {
    pub fn new(name: String, args: Vec<String>, node: Node) -> FunctionSymbol {
        FunctionSymbol {
            name,
            args,
            node,
        }
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match &self.value {
            SymbolType::Integer(value) => value.to_string(),
            SymbolType::Float(value) => value.to_string(),
            SymbolType::String(value) => value.to_string(),
            SymbolType::Node(value) => value.to_string(),
            SymbolType::Function(value) => value.to_string(),
            SymbolType::None => "None".to_string(),
        }
    }
}

impl std::fmt::Display for FunctionSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionSymbol: {}", self.name)
    }
}

impl std::fmt::Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolType::Integer(value) => write!(f, "{}", value),
            SymbolType::Float(value) => write!(f, "{}", value),
            SymbolType::String(value) => write!(f, "{}", value),
            SymbolType::Node(value) => write!(f, "{}", value),
            SymbolType::Function(value) => write!(f, "{}", value),
            SymbolType::None => write!(f, "None"),
        }
    }
}