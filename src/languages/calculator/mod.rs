// pub mod interpreter;
pub mod lexer;
// pub mod parser;

#[derive(PartialEq, Debug)]
pub enum Token {
    TokPlus,
    TokSub,
    TokMult,
    TokDiv,
    TokNumber(i32),
}
