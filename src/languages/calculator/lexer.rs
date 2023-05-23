use crate::languages::calculator::Token;
use regex::Regex;

pub fn lexer(input: &String, pos: usize) -> Vec<Token> {
    match (input, pos) {
        (_, pos) if pos >= input.len() => {
            let empty: Vec<Token> = Vec::new();
            empty
        }
        (input, pos) if Regex::new(r"\+").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokPlus);
            tokens
        }
        (input, pos) if Regex::new(r"\*").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokMult);
            tokens
        }
        (input, pos) if Regex::new(r"\-").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokSub);
            tokens
        }
        (input, pos) if Regex::new(r"\/").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokDiv);
            tokens
        }
        _ => {
            let re_whitespace = Regex::new(r"(\s+)").unwrap();
            if let Some(captures) = re_whitespace.captures(&input[pos..]) {
                if let Some(matched) = captures.get(0) {
                    let matched_str = matched.as_str();
                    return lexer(input, pos + matched_str.len());
                } else {
                    panic!("invalid token");
                }
            } else {
                panic!("invalid token");
            }
        }
    }
}
