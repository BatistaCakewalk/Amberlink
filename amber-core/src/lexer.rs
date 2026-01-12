// amber-core/src/lexer.rs

#[derive(Debug, PartialEq)]
pub enum Token {
    Val, Mut, Func, Class,    // Keywords
    Identifier(String),        // variable/function names
    Number(i64),
    StringLit(String),
    Equals, Plus, Minus,       // Operators
    OpenParen, CloseParen,
    Newline,                   // Essential for our "Better Java" logic
    EOF,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        // 1. Skip spaces, but NOT newlines
        // 2. If it's '\n', return Token::Newline
        // 3. If it's a letter, check if it's "val" or "func"
        // 4. If it's a digit, parse a Number
        Token::EOF 
    }
}
