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
            Token::Var => self.parse_declaration(),
            Token::Int | Token::Void | Token::String => {
                // Lookahead to distinguish Variable Declaration vs Function Definition
                // int x = 5;       (Type -> Identifier -> Equals)
                // int x() { ... }  (Type -> Identifier -> LParen)
                if matches!(self.peek_n(1), Token::Identifier(_)) && self.peek_n(2) == Token::LParen {
                    self.parse_function(symbols)
                } else {
                    self.parse_declaration()
                }
            }
            Token::If => self.parse_if(symbols),
            Token::While => self.parse_while(symbols),
            Token::LBrace => self.parse_block(symbols),
            Token::Return => self.parse_return(),
            Token::Print => self.parse_print(),
            // Token::Func is deprecated in favor of C-style types
            Token::Identifier(_) => {
                // Parse as expression first to handle L-values (Variable or ArrayAccess)
                let expr = self.parse_expr();
                
                if self.peek() == Token::Equals {
                    self.advance(); // consume '='
                    let value = self.parse_expr();
                    match expr {
                        Expr::Variable(name) => Stmt::Assign(name, value),
                        Expr::ArrayAccess(name, index) => Stmt::ArraySet(name, *index, value),
                        _ => panic!("Invalid assignment target. Only variables and array elements can be assigned."),
                    }
                } else {
                    Stmt::Expression(expr)
                }
            }
            _ => Stmt::Expression(self.parse_expr()),
        }
    }

    // --- Expression Parsing (Recursive Descent) ---

    fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();
        while matches!(self.peek(), Token::LessThan) {
            self.advance(); // consume '<'
            let right = self.parse_term();
            expr = Expr::Binary(Box::new(expr), Op::LessThan, Box::new(right));
        }
        expr
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
            Token::New => {
                // new int[size]
                if !matches!(self.advance(), Token::Int | Token::String) { panic!("Expected type after new"); }
                if self.advance() != Token::LBracket { panic!("Expected '[' after type"); }
                let size = self.parse_expr();
                if self.advance() != Token::RBracket { panic!("Expected ']' after size"); }
                Expr::NewArray(Box::new(size))
            }
            Token::StringLit(s) => Expr::StringLiteral(s),
            Token::Identifier(name) => {
                if self.peek() == Token::LParen {
                    self.advance(); // skip '('
                    let mut args = Vec::new();
                    if self.peek() != Token::RParen {
                        loop {
                            args.push(self.parse_expr());
                            if self.peek() == Token::Comma { self.advance(); } else { break; }
                        }
                    }
                    if self.advance() != Token::RParen {
                        panic!("Expected ')' after arguments");
                    }
                    Expr::Call(name, args)
                } else if self.peek() == Token::LBracket {
                    self.advance(); // [
                    let index = self.parse_expr();
                    if self.advance() != Token::RBracket { panic!("Expected ']'"); }
                    Expr::ArrayAccess(name, Box::new(index))
                } else {
                    Expr::Variable(name)
                }
            }
            tok => panic!(
                "Unexpected token in expression: {:?}. Expected a number or identifier.",
                tok
            ),
        }
    }

    fn parse_function(&mut self, symbols: &mut SymbolTable) -> Stmt {
        self.advance(); // consume Return Type (int/void)

        let name_token = self.advance();
        let name = match name_token {
            Token::Identifier(n) => n,
            _ => panic!("Expected function name, found {:?}", name_token),
        };

        // Parse Parameters
        if self.advance() != Token::LParen { panic!("Expected '(' after function name"); }
        let mut params = Vec::new();
        if self.peek() != Token::RParen {
            loop {
                // Parse Parameter Type (e.g., "int")
                if !matches!(self.peek(), Token::Int | Token::Void | Token::String) { panic!("Expected parameter type"); }
                self.advance(); 

                match self.advance() {
                    Token::Identifier(param) => params.push(param),
                    _ => panic!("Expected parameter name"),
                }
                if self.peek() == Token::Comma { self.advance(); } else { break; }
            }
        }
        if self.advance() != Token::RParen { panic!("Expected ')' after parameters"); }

        // Register function in the symbol table (Pass 1: Discovery)
        symbols.functions.insert(name.clone(), crate::semant::FunctionInfo {
            name: name.clone(),
            address: 0, // Placeholder: Will be resolved during emission
        });

        // Setup scope for function body
        let old_locals = symbols.locals.clone();
        let old_local_index = symbols.next_local_index;
        symbols.locals.clear();
        symbols.next_local_index = 0;

        // Register parameters as locals
        for param in &params {
            symbols.locals.insert(param.clone(), symbols.next_local_index);
            symbols.next_local_index += 1;
        }

        // Parse Body
        let body_stmt = self.parse_block(symbols);
        let body = match body_stmt { Stmt::Block(stmts) => stmts, _ => vec![] };

        // Restore scope
        symbols.locals = old_locals;
        symbols.next_local_index = old_local_index;

        Stmt::Function(name, params, body)
    }

    fn parse_declaration(&mut self) -> Stmt {
        self.advance(); // consume Type (int/var)
        
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

    fn parse_return(&mut self) -> Stmt {
        self.advance(); // skip 'return'
        let value = self.parse_expr();
        Stmt::Return(value)
    }

    fn parse_print(&mut self) -> Stmt {
        self.advance(); // skip 'print'
        let expr = self.parse_expr();
        Stmt::Print(expr)
    }

    fn peek(&self) -> Token { self.tokens[self.pos].clone() }
    fn advance(&mut self) -> Token { 
        let tok = self.tokens[self.pos].clone();
        if !self.is_at_end() { self.pos += 1; }
        tok
    }
    fn peek_n(&self, n: usize) -> Token {
        if self.pos + n >= self.tokens.len() { return Token::EOF; }
        self.tokens[self.pos + n].clone()
    }
    fn is_at_end(&self) -> bool { self.peek() == Token::EOF }
}
