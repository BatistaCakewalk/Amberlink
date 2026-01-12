// amber-core/src/parser.rs
use crate::lexer::Token;
use crate::semant::SymbolTable;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self, symbols: &mut SymbolTable) {
        while !self.is_at_end() {
            match self.peek() {
                Token::Func => self.parse_function(symbols),
                Token::Newline => { self.advance(); }
                _ => { self.advance(); }
            }
        }
    }

    fn parse_function(&mut self, _symbols: &mut SymbolTable) {
        self.advance(); // skip 'func'
        // Logic to extract name and parameters would go here
    }

    fn peek(&self) -> Token { self.tokens[self.pos].clone() }
    fn advance(&mut self) { self.pos += 1; }
    fn is_at_end(&self) -> bool { self.peek() == Token::EOF }
}
