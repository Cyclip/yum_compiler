//! Different nodes within the AST to be interpreted
#![allow(dead_code)]

use crate::{
    lexer::tokens::Token, 
};

// ======================== Base Node Enum ========================
/// Different parser nodes
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
}

// =========================== All nodes ===========================
/// Statements node
#[derive(Debug)]
pub struct StatementsNode {
    pub statements: Vec<Node>,
}

/// Number (int/float) node
#[derive(Debug)]
pub struct NumberNode {
    token: Token,
}

/// String node (includes chars)
#[derive(Debug)]
pub struct StringNode {
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

/// Variable arithmetic assignment node
#[derive(Debug)]
pub struct VarArithmeticAssignmentNode {
    identifier: Token,
    op_token: Token,
    value: Node,
}

/// Variable assignment node
#[derive(Debug)]
pub struct VarAccessNode {
    identifier: Token,
}

/// If statement expression node
#[derive(Debug)]
pub struct IfExprNode {
    condition: Node,
    if_true: Node,
    if_false: Option<Node>,
}

/// Function definition node
#[derive(Debug)]
pub struct FuncDefNode {
    identifier: Token,
    parameters: Vec<Token>,
    body: Option<Node>,
}

/// Function call node
#[derive(Debug)]
pub struct FuncCallNode {
    func_node: Node,
    args: Vec<Node>,
}

/// List expression node
#[derive(Debug)]
pub struct ListExprNode {
    elements: Vec<Node>,
}

/// Return statement node
#[derive(Debug)]
pub struct ReturnNode {
    value: Option<Node>,
}

// =========================== Node impl ===========================
impl StatementsNode {
    pub fn new(statements: Vec<Node>) -> StatementsNode {
        StatementsNode { statements }
    }
}

impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode { token }
    }
}

impl StringNode {
    pub fn new(token: Token) -> StringNode {
        StringNode { token }
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

impl VarArithmeticAssignmentNode {
    pub fn new(identifier: Token, op_token: Token, value: Node) -> VarArithmeticAssignmentNode {
        VarArithmeticAssignmentNode {
            identifier,
            op_token,
            value,
        }
    }
}

impl VarAccessNode {
    pub fn new(identifier: Token) -> VarAccessNode {
        VarAccessNode { identifier }
    }
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

impl FuncDefNode {
    pub fn new(identifier: Token, parameters: Vec<Token>, body: Option<Node>) -> FuncDefNode {
        FuncDefNode {
            identifier,
            parameters,
            body,
        }
    }
}

impl FuncCallNode {
    pub fn new(func_node: Node, args: Vec<Node>) -> FuncCallNode {
        FuncCallNode {
            func_node,
            args,
        }
    }
}

impl ListExprNode {
    pub fn new(elements: Vec<Node>) -> ListExprNode {
        ListExprNode {
            elements,
        }
    }
}

impl ReturnNode {
    pub fn new(value: Option<Node>) -> ReturnNode {
        ReturnNode {
            value: value,
        }
    }
}