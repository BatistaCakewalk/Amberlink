#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i32),
    Variable(String),
    Binary(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(String, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>), // Condition, Then, Else
    While(Expr, Box<Stmt>),                 // Condition, Body
    Expression(Expr),
    Function(String), // Placeholder for now
}