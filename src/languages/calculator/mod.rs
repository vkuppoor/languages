pub mod interpreter;
pub mod lexer;
pub mod parser;
use super::error;
use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum CalculatorError {
    Parser(error::parser::Error<Tok, Vec<Tok>>),
    Lexer(error::lexer::Error<String>),
    Interpreter(error::interpreter::Error),
}

impl<'a> std::error::Error for CalculatorError {}

impl<'a> fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::Parser(err) => write!(f, "Parser error: {:?}", err),
            CalculatorError::Lexer(err) => write!(f, "Lexer error: {:?}", err),
            CalculatorError::Interpreter(err) => write!(f, "Interpreter error: {:?}", err),
        }
    }
}
