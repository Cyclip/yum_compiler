#[allow(unused_imports)]
use super::{Node, NodeVisit, string_parameters, get_name_as_string};
use crate::{interpreter::{symbol_table::SymbolTable, symbols::SymbolType}, lexer::tokens::TokenPosition};
#[allow(unused_imports)]
use crate::{interpreter::symbols::Symbol, lexer::tokens::Token, errors::{Error, ErrorType}};

/// Function call node
#[derive(Debug, Clone)]
pub struct FuncCallNode {
    pub func_node: Node,
    pub args: Vec<Node>,
}

impl FuncCallNode {
    pub fn new(func_node: Node, args: Vec<Node>) -> FuncCallNode {
        FuncCallNode {
            func_node,
            args,
        }
    }
}

impl NodeVisit for FuncCallNode {
    fn visit(&self, symbol_table: &mut SymbolTable) -> Result<Symbol, Error> {
        // evaluate arguments into symbols
        println!("Calling func_node: {:#?}", &self.func_node);
        let mut args: Vec<Symbol> = Vec::new();

        for arg in &self.args {
            args.push(arg.visit(symbol_table)?);
        };

        // create a new symbol table for the function call
        let mut func_symbol_table = SymbolTable::new(Box::new(&*symbol_table));
        // println!("func_symbol_table: {:?}", func_symbol_table);

        // get function node and evaluate
        let func_identifier = match self.func_node {
            Node::VarAccessNode(ref var_access_node) => get_name_as_string(var_access_node.identifier.clone())?,
            _ => return Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot call non-function {:?}", self.func_node),
                &self.func_node.get_position()
            ))
        };

        let func_symbol = match symbol_table.get(&func_identifier) {
            Some(symbol) => {
                match symbol.value {
                    SymbolType::Function(func_symbol) => {
                        let statements_node = match func_symbol.node {
                            Node::StatementsNode(statements_node) => statements_node,
                            _ => panic!("Function node is not a StatementsNode")
                        };

                        // visit all statements until a return statement is found
                        for statement in statements_node.statements {
                            match statement {
                                Node::ReturnNode(ref return_node) => {
                                    return return_node.visit(&mut func_symbol_table);
                                },
                                _ => {
                                    statement.visit(&mut func_symbol_table)?;
                                }
                            }
                        }
                    },
                    _ => return Err(Error::new_runtime(
                        ErrorType::TypeError, 
                        format!("Cannot call non-function {:?}", self.func_node),
                        &self.func_node.get_position()
                    ))
                };
                Symbol::new(SymbolType::None, self.get_position())
            },
            None => Symbol::new(SymbolType::None, self.get_position())
        };

        Ok(func_symbol)

    }

    fn get_position(&self) -> TokenPosition {
        self.func_node.get_position()
    }
}

