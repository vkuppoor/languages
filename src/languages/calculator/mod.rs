pub mod interpreter;
pub mod lexer;
pub mod parser;

#[derive(PartialEq, Debug, Clone)]
pub enum Tok {
    TokAdd,
    TokSub,
    TokMult,
    TokDiv,
    TokInt(i32),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Add((Box<Expr>, Box<Expr>)),
    Sub((Box<Expr>, Box<Expr>)),
    Mult((Box<Expr>, Box<Expr>)),
    Div((Box<Expr>, Box<Expr>)),
    Int(i32),
}
