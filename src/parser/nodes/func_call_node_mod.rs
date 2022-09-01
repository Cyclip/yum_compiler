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
        // arguments will be set after function node is found
        let mut func_symbol_table = SymbolTable::new(Box::new(&*symbol_table));
        // println!("func_symbol_table: {:?}", func_symbol_table);

        // get function node
        let func_identifier = match self.func_node {
            Node::VarAccessNode(ref var_access_node) => get_name_as_string(var_access_node.identifier.clone())?,
            _ => return Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Cannot call non-function {:?}", self.func_node),
                &self.func_node.get_position()
            ))
        };

        // get the function node
        let func_node = match symbol_table.get(&func_identifier) {
            Some(symbol) => match symbol.value {
                SymbolType::Function(func_symbol) => func_symbol,
                _ => return Err(Error::new_runtime(
                    ErrorType::TypeError, 
                    format!("Cannot call non-function {:?}", self.func_node),
                    &self.func_node.get_position()
                ))
            },
            None => return Err(Error::new_runtime(
                ErrorType::UndefinedVariable, 
                format!("Function {} not found", func_identifier),
                &self.func_node.get_position()
            ))
        };

        // ensure the right number of arguments are passed
        if func_node.args.len() != args.len() {
            return Err(Error::new_runtime(
                ErrorType::TypeError, 
                format!("Expected {} arguments, got {}", func_node.args.len(), args.len()),
                &self.func_node.get_position()
            ))
        }

        // set arguments in function symbol table
        func_symbol_table.set_args(&func_node.args, args);

        // evaluate function symbol
        let func_symbol = match func_node.node {
            // built in
            Node::ExecuteBuiltinNode(mut execute_builtin_node) => {
                execute_builtin_node.args = Some(self.args.clone());
                execute_builtin_node.visit(&mut func_symbol_table)?
            },

            // custom function
            Node::StatementsNode(statements) => {
                // visit all statements until a return statement is found
                for statement in statements.statements {
                    match statement {
                        Node::ReturnNode(ref return_node) => {
                            return return_node.visit(&mut func_symbol_table);
                        },
                        _ => {
                            statement.visit(&mut func_symbol_table)?;
                        }
                    }
                }

                // return none if no return statement
                Symbol::new(SymbolType::None, self.get_position())
            },
            _ => panic!("Function node must be a StatementsNode")
        };

        Ok(func_symbol)

    }

    fn get_position(&self) -> TokenPosition {
        self.func_node.get_position()
    }
}

impl ToString for FuncCallNode {
    fn to_string(&self) -> String {
        let mut args: Vec<String> = Vec::new();

        for arg in &self.args {
            args.push(arg.to_string());
        };

        format!("{}({})", self.func_node.to_string(), args.join(", "))
    }
}