use super::Tok;
use regex::Regex;

pub fn lexer(input: &String, pos: usize) -> Vec<Tok> {
    match (input, pos) {
        (_, pos) if pos >= input.len() => {
            let mut tokens: Vec<Tok> = Vec::new();
            tokens.insert(0, Tok::TokEOF);
            tokens
        }
        (input, pos) if Regex::new(r"^\+").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Tok> = lexer(input, pos + 1);
            tokens.insert(0, Tok::TokPlus);
            tokens
        }
        (input, pos) if Regex::new(r"^\*").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Tok> = lexer(input, pos + 1);
            tokens.insert(0, Tok::TokMult);
            tokens
        }
        (input, pos) if Regex::new(r"^\-").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Tok> = lexer(input, pos + 1);
            tokens.insert(0, Tok::TokSub);
            tokens
        }
        (input, pos) if Regex::new(r"^\/").unwrap().is_match(&input[pos..]) => {
            let mut tokens: Vec<Tok> = lexer(input, pos + 1);
            tokens.insert(0, Tok::TokDiv);
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
                            more_tokens.insert(0, Tok::TokInt(number));
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
                Tok::TokInt(5),
                Tok::TokInt(4),
                Tok::TokInt(3),
                Tok::TokInt(11234),
                Tok::TokInt(9)
            ]
        );
    }

    #[test]
    fn operators() {
        assert_eq!(
            lexer(&String::from(" + - + - /* *  -- ++ // **"), 0),
            vec![
                Tok::TokPlus,
                Tok::TokSub,
                Tok::TokPlus,
                Tok::TokSub,
                Tok::TokDiv,
                Tok::TokMult,
                Tok::TokMult,
                Tok::TokSub,
                Tok::TokSub,
                Tok::TokPlus,
                Tok::TokPlus,
                Tok::TokDiv,
                Tok::TokDiv,
                Tok::TokMult,
                Tok::TokMult,
            ]
        );
    }

    #[test]
    fn numbers_operators() {
        assert_eq!(
            lexer(&String::from("+ 5 * 4 / 30 - 6 3"), 0),
            vec![
                Tok::TokPlus,
                Tok::TokInt(5),
                Tok::TokMult,
                Tok::TokInt(4),
                Tok::TokDiv,
                Tok::TokInt(30),
                Tok::TokSub,
                Tok::TokInt(6),
                Tok::TokInt(3)
            ]
        );
    }
}
