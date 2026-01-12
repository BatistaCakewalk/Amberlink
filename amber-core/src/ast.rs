#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    LessThan,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i32),
    StringLiteral(String),
    Variable(String),
    Call(String, Vec<Expr>),
    Binary(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(String, Expr),
    Return(Expr),
    Print(Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>), // Condition, Then, Else
    While(Expr, Box<Stmt>),                 // Condition, Body
    Expression(Expr),
    Function(String, Vec<String>, Vec<Stmt>), // Name, Params, Body
}