use super::Token;
use regex::Regex;

pub fn lexer(input: &String, pos: usize) -> Vec<Token> {
    match (input, pos) {
        (_, pos) if pos >= input.len() => {
            let empty: Vec<Token> = Vec::new();
            empty
        }
        (input, pos) if Regex::new(r"^\+").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokPlus);
            tokens
        }
        (input, pos) if Regex::new(r"^\*").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokMult);
            tokens
        }
        (input, pos) if Regex::new(r"^\-").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokSub);
            tokens
        }
        (input, pos) if Regex::new(r"^\/").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Token> = lexer(input, pos + 1);
            tokens.insert(0, Token::TokDiv);
            tokens
        }
        _ => {
            let re_whitespace = Regex::new(r"^(\s+)").unwrap();
            let re_numbers = Regex::new(r"^([0-9]+)").unwrap();
            if let Some(captures) = re_whitespace.captures(&input[pos..]) {
                if let Some(matched) = captures.get(0) {
                    let matched_str = matched.as_str();
                    lexer(input, pos + matched_str.len())
                } else {
                    panic!("invalid token");
                }
            } else if let Some(captures) = re_numbers.captures(&input[pos..]) {
                if let Some(matched) = captures.get(0) {
                    let matched_str = matched.as_str();

                    match matched_str.parse::<i32>() {
                        Ok(number) => {
                            let mut more_tokens = lexer(input, pos + matched_str.len());
                            more_tokens.insert(0, Token::TokInt(number));
                            more_tokens
                        }
                        Err(_) => panic!("invalid number token"),
                    }
                } else {
                    panic!("invalid token");
                }
            } else {
                panic!("invalid token");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing() {
        assert_eq!(lexer(&String::from(""), 0), Vec::new());
    }

    #[test]
    fn numbers() {
        assert_eq!(
            lexer(&String::from("5 4 3 11234 9"), 0),
            vec![
                Token::TokInt(5),
                Token::TokInt(4),
                Token::TokInt(3),
                Token::TokInt(11234),
                Token::TokInt(9)
            ]
        );
    }

    #[test]
    fn operators() {
        assert_eq!(
            lexer(&String::from(" + - + - /* *  -- ++ // **"), 0),
            vec![
                Token::TokPlus,
                Token::TokSub,
                Token::TokPlus,
                Token::TokSub,
                Token::TokDiv,
                Token::TokMult,
                Token::TokMult,
                Token::TokSub,
                Token::TokSub,
                Token::TokPlus,
                Token::TokPlus,
                Token::TokDiv,
                Token::TokDiv,
                Token::TokMult,
                Token::TokMult,
            ]
        );
    }

    #[test]
    fn numbers_operators() {
        assert_eq!(
            lexer(&String::from("+ 5 * 4 / 30 - 6 3"), 0),
            vec![
                Token::TokPlus,
                Token::TokInt(5),
                Token::TokMult,
                Token::TokInt(4),
                Token::TokDiv,
                Token::TokInt(30),
                Token::TokSub,
                Token::TokInt(6),
                Token::TokInt(3)
            ]
        );
    }
}
