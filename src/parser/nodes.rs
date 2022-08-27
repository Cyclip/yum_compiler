//! Different nodes within the AST to be interpreted

use crate::{
    lexer::{tokens::{Token, TokenType}, LexerPosition}, 
};

// ======================== Base Node Enum ========================
/// Different parser nodes
#[derive(Debug)]
pub enum Node {
    Number(Box<NumberNode>),
    BinOp(Box<BinOpNode>),
    UnaryOpNode(Box<UnaryOpNode>),
    VarAssignmentNode(Box<VarAssignmentNode>),
    VarAccessNode(Box<VarAccessNode>),
}

// =========================== All nodes ===========================
/// Number (int/float) node
#[derive(Debug)]
pub struct NumberNode {
    token: Token,
}

/// Unary operation node
#[derive(Debug)]
pub struct UnaryOpNode {
    token: Token,
    right: Node,
}

/// Binary operation node
#[derive(Debug)]
pub struct BinOpNode {
    left_node: Node,
    op_token: Token,
    right_node: Node,
}

/// Variable assignment node
#[derive(Debug)]
pub struct VarAssignmentNode {
    identifier: Token,
    value: Node,
}

/// Variable assignment node
#[derive(Debug)]
pub struct VarAccessNode {
    identifier: Token,
}

// =========================== Node impl ===========================
impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode { token }
    }
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

impl UnaryOpNode {
    pub fn new(token: Token, right: Node) -> UnaryOpNode {
        UnaryOpNode {
            token,
            right: right,
        }
    }
}

impl VarAssignmentNode {
    pub fn new(identifier: Token, value: Node) -> VarAssignmentNode {
        VarAssignmentNode {
            identifier,
            value,
        }
    }
}

impl VarAccessNode {
    pub fn new(identifier: Token) -> VarAccessNode {
        VarAccessNode { identifier }
    }
}