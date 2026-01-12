// amber-core/src/parser.rs
use crate::lexer::Token;
use crate::semant::SymbolTable;
use crate::ast::{Stmt, Expr, Op};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self, symbols: &mut SymbolTable) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.peek() {
                Token::Func => statements.push(self.parse_function(symbols)),
                Token::Val => statements.push(self.parse_declaration()),
                Token::Newline => { self.advance(); }
                _ => statements.push(Stmt::Expression(self.parse_expr())),
            }
        }
        statements
    }

    // --- Expression Parsing (Recursive Descent) ---

    fn parse_expr(&mut self) -> Expr {
        self.parse_term()
    }

    // Handles + and -
    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => Op::Add,
                Token::Minus => Op::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    // Handles * and /
    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        while matches!(self.peek(), Token::Star | Token::Slash) {
            let op = match self.advance() {
                Token::Star => Op::Mul,
                Token::Slash => Op::Div,
                _ => unreachable!(),
            };
            let right = self.parse_primary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Token::Number(val) => Expr::Integer(val as i32),
            Token::Identifier(name) => Expr::Variable(name),
            _ => panic!("Expected expression, found {:?}", self.peek()),
        }
    }

    fn parse_function(&mut self, symbols: &mut SymbolTable) -> Stmt {
        self.advance(); // skip 'func'
        
        let name_token = self.advance();
        let name = match name_token {
            Token::Identifier(n) => n,
            _ => panic!("Expected function name, found {:?}", name_token),
        };

        // Register function in the symbol table (Pass 1: Discovery)
        symbols.functions.insert(name.clone(), crate::semant::FunctionInfo {
            name: name.clone(),
            address: 0, // Placeholder: Will be resolved during emission
        });

        // TODO: Parse parameters and body block
        Stmt::Function(name)
    }

    fn parse_declaration(&mut self) -> Stmt {
        self.advance(); // skip 'val'
        
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => panic!("Expected variable name"),
        };

        if self.advance() != Token::Equals { panic!("Expected '=' after variable name"); }
        
        let initializer = self.parse_expr();
        Stmt::VarDecl(name, initializer)
    }

    fn peek(&self) -> Token { self.tokens[self.pos].clone() }
    fn advance(&mut self) -> Token { 
        let tok = self.tokens[self.pos].clone();
        if !self.is_at_end() { self.pos += 1; }
        tok
    }
    fn is_at_end(&self) -> bool { self.peek() == Token::EOF }
}
