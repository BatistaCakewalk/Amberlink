// amber-core/src/lexer.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Var, Mut, Func, Class, Return, Print,
    Int, Void, String, New, // Types & Keywords
    If, Else, While,
    Identifier(String),
    Number(i64),
    StringLit(String),
    Equals, Plus, Minus, Star, Slash, Comma, Dot, LessThan,
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Newline,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            match c {
                ' ' | '\r' | '\t' => { self.pos += 1; }
                '\n' => { tokens.push(Token::Newline); self.pos += 1; }
                '=' => { tokens.push(Token::Equals); self.pos += 1; }
                '+' => { tokens.push(Token::Plus); self.pos += 1; }
                '-' => { tokens.push(Token::Minus); self.pos += 1; }
                '*' => { tokens.push(Token::Star); self.pos += 1; }
                '/' => { 
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '/' {
                        while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                            self.pos += 1;
                        }
                    } else {
                        tokens.push(Token::Slash); self.pos += 1; 
                    }
                }
                '<' => { tokens.push(Token::LessThan); self.pos += 1; }
                '.' => { tokens.push(Token::Dot); self.pos += 1; }
                ',' => { tokens.push(Token::Comma); self.pos += 1; }
                '(' => { tokens.push(Token::LParen); self.pos += 1; }
                ')' => { tokens.push(Token::RParen); self.pos += 1; }
                '{' => { tokens.push(Token::LBrace); self.pos += 1; }
                '}' => { tokens.push(Token::RBrace); self.pos += 1; }
                '[' => { tokens.push(Token::LBracket); self.pos += 1; }
                ']' => { tokens.push(Token::RBracket); self.pos += 1; }
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.read_identifier()),
                '0'..='9' => tokens.push(self.read_number()),
                '"' => tokens.push(self.read_string()),
                _ => { self.pos += 1; } // Skip unknowns
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
            self.pos += 1;
        }
        let text: String = self.input[start..self.pos].iter().collect();
        match text.as_str() {
            "var" => Token::Var,
            "int" => Token::Int,
            "void" => Token::Void,
            "String" => Token::String,
            "new" => Token::New,
            "mut" => Token::Mut,
            "func" => Token::Func,
            "class" => Token::Class,
            "return" => Token::Return,
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            _ => Token::Identifier(text),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
            self.pos += 1;
        }
        let text: String = self.input[start..self.pos].iter().collect();
        Token::Number(text.parse().unwrap())
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1; // Skip opening quote
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.pos += 1;
        }
        let text: String = self.input[start..self.pos].iter().collect();
        if self.pos < self.input.len() { self.pos += 1; } // Skip closing quote
        Token::StringLit(text)
    }
}
