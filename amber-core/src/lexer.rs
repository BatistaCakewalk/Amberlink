// amber-core/src/lexer.rs

pub enum Token {
    Keyword(String), // e.g., "func", "val"
    Identifier(String),
    Number(i64),
    Operator(char),
    EOF,
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        // Logic to skip whitespace and identify tokens
        // This is the "brain" that sees 'print' and knows it's a command
        Token::EOF // Placeholder
    }
}
