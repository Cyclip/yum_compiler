//! Interpreter for the programming language

pub mod symbols;
pub mod symbol_table;

use symbol_table::SymbolTable;

use crate::{parser::nodes::{Node, NodeVisit}, errors::{Error}};

pub struct Interpreter {
    pub symbol_table: SymbolTable,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            symbol_table: SymbolTable::new_global(),
        }
    }

    pub fn run(&mut self, ast: Node) -> Result<(), Error> {
        // first node must be a statements node
        let statements = match ast {
            Node::StatementsNode(statements) => *statements,
            _ => panic!("First node must be a statements node"),
        };

        // run all statements in order
        for statement in statements.statements {
            statement.visit(&mut self.symbol_table)?;
        }

        Ok(())
    }
}

mod operations;