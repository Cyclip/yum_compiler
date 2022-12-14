//! Different nodes within the AST to be interpreted
#![allow(dead_code)]

pub mod number_node_mod;
pub mod string_node_mod;
pub mod bin_op_node_mod;
pub mod unary_op_node_mod;
pub mod var_assignment_node_mod;
pub mod var_arithmetic_assignment_node_mod;
pub mod var_access_node_mod;
pub mod if_expr_node_mod;
pub mod func_def_node_mod;
pub mod func_call_node_mod;
pub mod list_expr_node_mod;
pub mod statements_node_mod;
pub mod return_node_mod;
pub mod assert_node_mod;
pub mod execute_builtin_mod;

pub use number_node_mod::NumberNode;
pub use string_node_mod::StringNode;
pub use bin_op_node_mod::BinOpNode;
pub use unary_op_node_mod::UnaryOpNode;
pub use var_assignment_node_mod::VarAssignmentNode;
pub use var_arithmetic_assignment_node_mod::VarArithmeticAssignmentNode;
pub use var_access_node_mod::VarAccessNode;
pub use if_expr_node_mod::IfExprNode;
pub use func_def_node_mod::FuncDefNode;
pub use func_call_node_mod::FuncCallNode;
pub use list_expr_node_mod::ListExprNode;
pub use statements_node_mod::StatementsNode;
pub use return_node_mod::ReturnNode;
pub use assert_node_mod::AssertNode;
pub use execute_builtin_mod::ExecuteBuiltinNode;

use crate::{
    errors::{Error, ErrorType}, 
    interpreter::{
        symbols::Symbol,
        symbol_table::SymbolTable,
    }, lexer::tokens::{TokenPosition, TokenType, Token},
};

/// Base node enum
#[derive(Debug, Clone)]
pub enum Node {
    NumberNode(Box<NumberNode>),
    StringNode(Box<StringNode>), 
    BinOpNode(Box<BinOpNode>),
    UnaryOpNode(Box<UnaryOpNode>),
    VarAssignmentNode(Box<VarAssignmentNode>),
    VarArithmeticAssignmentNode(Box<VarArithmeticAssignmentNode>),
    VarAccessNode(Box<VarAccessNode>),
    IfExprNode(Box<IfExprNode>),
    FuncDefNode(Box<FuncDefNode>),
    FuncCallNode(Box<FuncCallNode>),
    ListExprNode(Box<ListExprNode>),
    StatementsNode(Box<StatementsNode>),
    ReturnNode(Box<ReturnNode>),
    AssertNode(Box<AssertNode>),
    ExecuteBuiltinNode(Box<ExecuteBuiltinNode>),
}

/// Trait for the node to be able to be visited (evaluated)
pub trait NodeVisit {
    fn visit(&self, symbol_table: &mut SymbolTable) -> Result<Symbol, Error>;
    fn get_position(&self) -> TokenPosition;
}

/// Function to get the name of an identifier as a String
fn get_name_as_string(identifier: Token) -> Result<String, Error> {
    match identifier.value {
        TokenType::Identifier(s) => Ok(s),
        _ => Err(Error::new_runtime(
            ErrorType::TypeError, 
            "Expected identifier".to_string(), 
            &identifier.position
        ))
    }
}

/// Convert a vector of string tokens as strings
fn string_parameters(parameters: Vec<Token>) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();

    for parameter in &parameters {
        strings.push(get_name_as_string(parameter.clone()).unwrap());
    }

    strings
}

impl NodeVisit for Node {
    fn visit(&self, symbol_table: &mut SymbolTable) -> Result<Symbol, Error> {
        match self {
            Node::NumberNode(node) => node.visit(symbol_table),
            Node::StringNode(node) => node.visit(symbol_table),
            Node::BinOpNode(node) => node.visit(symbol_table),
            Node::UnaryOpNode(node) => node.visit(symbol_table),
            Node::VarAssignmentNode(node) => node.visit(symbol_table),
            Node::VarArithmeticAssignmentNode(node) => node.visit(symbol_table),
            Node::VarAccessNode(node) => node.visit(symbol_table),
            Node::IfExprNode(node) => node.visit(symbol_table),
            Node::FuncDefNode(node) => node.visit(symbol_table),
            Node::FuncCallNode(node) => node.visit(symbol_table),
            Node::ListExprNode(node) => node.visit(symbol_table),
            Node::ReturnNode(node) => node.visit(symbol_table),
            Node::AssertNode(node) => node.visit(symbol_table),
            Node::ExecuteBuiltinNode(node) => node.visit(symbol_table),
            Node::StatementsNode(_) => panic!("Cannot visit statements node"),
        }
    }

    fn get_position(&self) -> TokenPosition {
        match self {
            Node::NumberNode(node) => node.get_position(),
            Node::StringNode(node) => node.get_position(),
            Node::BinOpNode(node) => node.get_position(),
            Node::UnaryOpNode(node) => node.get_position(),
            Node::VarAssignmentNode(node) => node.get_position(),
            Node::VarArithmeticAssignmentNode(node) => node.get_position(),
            Node::VarAccessNode(node) => node.get_position(),
            Node::IfExprNode(node) => node.get_position(),
            Node::FuncDefNode(node) => node.get_position(),
            Node::FuncCallNode(node) => node.get_position(),
            Node::ListExprNode(node) => node.get_position(),
            Node::ReturnNode(node) => node.get_position(),
            Node::AssertNode(node) => node.get_position(),
            Node::ExecuteBuiltinNode(node) => node.get_position(),
            Node::StatementsNode(_) => panic!("Cannot visit statements node"),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Node::NumberNode(node) => node.to_string(),
            Node::StringNode(node) => node.to_string(),
            Node::BinOpNode(node) => node.to_string(),
            Node::UnaryOpNode(node) => node.to_string(),
            Node::VarAssignmentNode(node) => node.to_string(),
            Node::VarArithmeticAssignmentNode(node) => node.to_string(),
            Node::VarAccessNode(node) => node.to_string(),
            Node::IfExprNode(node) => node.to_string(),
            Node::FuncDefNode(node) => node.to_string(),
            Node::FuncCallNode(node) => node.to_string(),
            Node::ListExprNode(node) => node.to_string(),
            Node::ReturnNode(node) => node.to_string(),
            Node::AssertNode(node) => node.to_string(),
            Node::ExecuteBuiltinNode(node) => node.to_string(),
            Node::StatementsNode(_) => panic!("Cannot stringify statements node"),
        };

        write!(f, "{}", text)
    }
}