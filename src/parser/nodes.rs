use crate::lexer::tokens::Token;

/// Different parser nodes
#[derive(Debug)]
pub enum Node {
    Number(Box<NumberNode>),
    BinOp(Box<BinOpNode>),
}

/// Number (int/float) node
#[derive(Debug)]
pub struct NumberNode {
    token: Token,
}

/// Binary operation node
#[derive(Debug)]
pub struct BinOpNode {
    left_node: Node,
    op_token: Token,
    right_node: Node,
}

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