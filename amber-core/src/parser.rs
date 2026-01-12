// amber-core/src/parser.rs
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    // Pass 1: Scan for function and class names
    pub fn discovery_pass(&mut self, symbols: &mut SymbolTable) {
        while !self.is_at_end() {
            if self.peek() == Token::Func {
                self.consume(); // eat 'func'
                let name = self.consume_identifier();
                // ... extract params and return type ...
                symbols.define_func(name, FunctionInfo { ... });
            } else {
                self.advance();
            }
        }
        self.pos = 0; // Reset for Pass 2
    }

    // Pass 2: Actually build the Abstract Syntax Tree (AST)
    pub fn parse_statement(&mut self) -> ASTNode {
        match self.peek() {
            Token::Val => self.parse_variable_declaration(),
            Token::Func => self.parse_function_body(),
            _ => self.parse_expression(),
        }
    }
}
