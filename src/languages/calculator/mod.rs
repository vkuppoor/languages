// pub mod interpreter;
pub mod lexer;
// pub mod parser;

pub enum Token {
    TokPlus,
    TokSub,
    TokMult,
    TokDiv,
    TokInt(i8),
}
