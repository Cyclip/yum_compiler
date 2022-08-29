#[allow(unused_imports)]
use super::{Node, NodeVisit};
use crate::interpreter::symbol_table::SymbolTable;
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Assertion node
#[derive(Debug)]
pub struct AssertNode {
    pub condition: Node,
}

impl AssertNode {
    pub fn new(condition: Node) -> AssertNode {
        AssertNode {
            condition,
        }
    }
}

