# Yum interpreter
Rust-based interpreter for a new programming language. It doesn't have much features other than being a test language.  

## Test usage
Download the latest interpreter .exe from [releases](https://github.com/Cyclip/yum_compiler/releases).  
Command usage: `yum_compiler <code source>`  
Example: `yum_compiler ./test_code/login.yum`  

## Example syntax
### Variable declaration
`let <identifier> = <value>;`  
**Example**: `let pi_approximation = 3.1415926`  

### If statements
`if <condition> { expr } [else if <condition> { <expr> }]* [else { <expr> }]`  
**Example**: `if age > 18 { print("adult"); } else { print("child"); }  

### Function definitions and calling
`func <identifier>([<args>]*) { <expr> };`  
**Example**: `func incr(x) { let x += 1; return x; }`  

`<identifier>([<args>]*);`  
**Example**: `let a = incr(a);`

### Arithmetic and conditions
Basic arithmetic: `a + b`, `a - b`, `a * b`, `a / b`  
Assignment arithmetic: `a += b`, `a -= b`, `a *= b`, `a /= b`  
Power: `a^b`  
Negation: `-a`  
All gates can be applied to `a` and `b`: `<, <=, >, >=, ==, !=, not, and, or`

### Assertion
`assert <condition>;`  
**Example**: `assert 3 == 3;`

### Built-in functions
`print(<text>, <arguments>)`  
**Example**: `print("Hello {}!", "world");`  
**Example**: `print("Range: {} to {}", a, b);`  

`input(<prefix>)`  
**Example**: `input("Username: ");`

## Errors
**SyntaxError**: Invalid syntax when parsing
**InvalidToken**: Invalid or unexpected token
**ParserError**: Error during parsing
**AssertError**: Assertion is false
**InvalidOperation**: Operation between `a` and `b` is not supported
**Exception**: General exception
**TypeError**: Invalid type
**UndefinedVariable**: Variable or function is not defined
**ArgumentError**: Error with arguments passed into a function
**IOError**: Error when working with stdout/stdin

## Grammar
Located in `./src/grammar.txt`  
```
<statements>    ::= <statement> (<statement>)*

<statement>     ::= return <expr>?;
                ::= expr;

<expr>          ::= let <identifier> ('=' | '+=' | '-=' | '*=' | '/=') <expr>
                ::= assert <expr>
                ::= <compare-expr> (('and' | 'or') <compare-expr>)*
                
<compare-expr>  ::= (not) <compare-expr>
                ::= <arith-expr> (('==' | '!=' | '<' | '<=' | '>' | '>) <arith-expr>)*

<arith-expr>    ::= <term> (('+' | '-') <term>)*

<term>          ::= <factor> (('*' | '/') <factor>)*

<factor>        ::= call ('^' <factor>)*

<call>          ::= atom (<expr> (',' <expr>)* )?

<atom>          ::= INT/LONGINT/FLOAT/DOUBLE
                ::= ('+' | '-') <atom>
                ::= '(' <expr> ')'
                ::= '"' STRING '"'
                ::= <list-expr>
                ::= <if-expr>
                ::= <func-def>

<list-expr>     ::= '[' (<expr> (',' <expr>)* )? ']'

<if-expr>       ::= if <expr> { <expr> }
                        (elif { <expr> })*
                        (else { <expr> })?

<func-def>      ::= func <identifier>( (<identifier> (',' <identifier>)* )? ) { statements }
```

## Nodes
**NumberNode**: Contains a number (integer or float)
**StringNode**: Contains a string
**BinOpNode**: Node for any binary operation between 2 values (i.e. addition, and)
**UnaryOpNode**: Node for any unary operation (i.e. negation)
**VarAssignmentNode**: Node for variable assignment within the current scope
**VarArithmeticAssignmentNode**: Node for variable assignment after an arithmetic operation
**VarAccessNode**: Node to access a variable
**IfExprNode**: Node for if statement expressions
**FuncDefNode**: Node for function definitions
**FuncCallNode**: Node for function calling
**ListExprNode**: Node for list expressions
**StatementsNode**: Node for all available statements
**ReturnNode**: Node to return within a function expression
**AssertNode**: Node for assertion
**ExecuteBuiltinNode**: Node to execute a built-in function

## File structure
```
src
│   errors.rs                                       // Error structs reside here
│   examples.yum
│   grammar.txt                                     // Grammar text here
│   main.rs                                         // Handles the overall compilation
│
├───interpreter                                     // Interprets an AST
│   │   mod.rs
│   │   symbols.rs                                  // Values available after evaluating nodes
│   │   symbol_table.rs                             // Table of variables within a scope
│   │
│   ├───builtin                                     // All built-in functions
│   │       input.rs                                // Input function
│   │       mod.rs
│   │       print.rs                                // Print function
│   │
│   └───operations                                  // Operations which can be applied to symbols
│           add.rs
│           and.rs
│           div.rs
│           eq.rs
│           ge.rs
│           gt.rs
│           le.rs
│           lt.rs
│           mod.rs
│           mul.rs
│           ne.rs
│           neg.rs
│           not.rs
│           or.rs
│           power.rs
│           sub.rs
│
├───lexer                                           // Tokenizes a source string
│       mod.rs
│       tokens.rs                                   // Enums of possible tokens and keywords
│
└───parser                                          // Parses token into an AST
    │   mod.rs
    │
    └───nodes                                       // Available nodes for tokens to be parsed into
            assert_node_mod.rs
            bin_op_node_mod.rs
            execute_builtin_mod.rs
            func_call_node_mod.rs
            func_def_node_mod.rs
            if_expr_node_mod.rs
            list_expr_node_mod.rs
            mod.rs
            number_node_mod.rs
            return_node_mod.rs
            statements_node_mod.rs
            string_node_mod.rs
            unary_op_node_mod.rs
            var_access_node_mod.rs
            var_arithmetic_assignment_node_mod.rs   
            var_assignment_node_mod.rs
```