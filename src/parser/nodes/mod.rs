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

use crate::{
    errors::Error, 
    interpreter::{
        symbols::Symbol,
        symbol_table::SymbolTable,
    },
};

/// Base node enum
#[derive(Debug)]
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
}

/// Trait for the node to be able to be visited (evaluated)
pub trait NodeVisit {
    fn visit(&self, symbol_table: SymbolTable) -> Result<Symbol, Error>;
}

impl NodeVisit for Node {
    fn visit(&self, symbol_table: SymbolTable) -> Result<Symbol, Error> {
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
            Node::StatementsNode(node) => node.visit(symbol_table),
            Node::ReturnNode(node) => node.visit(symbol_table),
            Node::AssertNode(node) => node.visit(symbol_table),
        }
    }
}