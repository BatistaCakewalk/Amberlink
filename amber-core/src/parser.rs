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
                // Skip empty lines between top-level statements
                Token::Newline => { self.advance(); }
                _ => statements.push(self.parse_statement(symbols)),
            }
        }
        statements
    }

    fn parse_statement(&mut self, symbols: &mut SymbolTable) -> Stmt {
        match self.peek() {
            Token::Val => self.parse_declaration(),
            Token::If => self.parse_if(symbols),
            Token::While => self.parse_while(symbols),
            Token::LBrace => self.parse_block(symbols),
            _ => Stmt::Expression(self.parse_expr()),
        }
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
            tok => panic!(
                "Unexpected token in expression: {:?}. Expected a number or identifier.",
                tok
            ),
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

    fn parse_block(&mut self, symbols: &mut SymbolTable) -> Stmt {
        self.advance(); // skip '{'
        let mut statements = Vec::new();
        
        while !self.is_at_end() && self.peek() != Token::RBrace {
            if self.peek() == Token::Newline { self.advance(); continue; }
            statements.push(self.parse_statement(symbols));
        }

        if self.peek() == Token::RBrace {
            self.advance(); // skip '}'
        } else {
            panic!("Expected '}}' after block");
        }
        
        Stmt::Block(statements)
    }

    fn parse_if(&mut self, symbols: &mut SymbolTable) -> Stmt {
        self.advance(); // skip 'if'
        let condition = self.parse_expr();
        let then_branch = Box::new(self.parse_statement(symbols));
        let mut else_branch = None;

        if self.peek() == Token::Else {
            self.advance();
            else_branch = Some(Box::new(self.parse_statement(symbols)));
        }

        Stmt::If(condition, then_branch, else_branch)
    }

    fn parse_while(&mut self, symbols: &mut SymbolTable) -> Stmt {
        self.advance(); // skip 'while'
        let condition = self.parse_expr();
        let body = Box::new(self.parse_statement(symbols));
        Stmt::While(condition, body)
    }

    fn peek(&self) -> Token { self.tokens[self.pos].clone() }
    fn advance(&mut self) -> Token { 
        let tok = self.tokens[self.pos].clone();
        if !self.is_at_end() { self.pos += 1; }
        tok
    }
    fn is_at_end(&self) -> bool { self.peek() == Token::EOF }
}
