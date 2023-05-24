use core::panic;

use super::{Expr, Tok};

/** Grammar:
E -> + N E | - N E | * N E | / N E | N
N -> -2,147,483,648 | -2,147,483,647 | -2,147,483,646 | ... | 2,147,483,647
*/
pub fn parser(toks: Vec<Tok>) -> Expr {
    let (toks, e) = parse_e(toks);
    if !toks.is_empty() {
        panic!("toks not empty after parsing")
    } else {
        e
    }
}

fn parse_e(toks: Vec<Tok>) -> (Vec<Tok>, Expr) {
    match toks.get(0).unwrap() {
        Tok::TokAdd => {
            let toks = match_token(&toks, Tok::TokAdd);
            let (toks, e1) = parse_e(toks);
            let (toks, e2) = parse_e(toks);
            (toks, Expr::Add((Box::new(e1), Box::new(e2))))
        }
        Tok::TokSub => {
            let toks = match_token(&toks, Tok::TokSub);
            let (toks, e1) = parse_e(toks);
            let (toks, e2) = parse_e(toks);
            (toks, Expr::Sub((Box::new(e1), Box::new(e2))))
        }
        Tok::TokMult => {
            let toks = match_token(&toks, Tok::TokMult);
            let (toks, e1) = parse_e(toks);
            let (toks, e2) = parse_e(toks);
            (toks, Expr::Mult((Box::new(e1), Box::new(e2))))
        }
        Tok::TokDiv => {
            let toks = match_token(&toks, Tok::TokDiv);
            let (toks, e1) = parse_e(toks);
            let (toks, e2) = parse_e(toks);
            (toks, Expr::Div((Box::new(e1), Box::new(e2))))
        }
        Tok::TokInt(_) => parse_n(toks),
    }
}

fn parse_n(toks: Vec<Tok>) -> (Vec<Tok>, Expr) {
    match toks.get(0).unwrap() {
        Tok::TokInt(i) => {
            let toks = match_token(&toks, Tok::TokInt(*i));
            (toks, Expr::Int(*i))
        }
        _ => panic!("Parse_N Error"),
    }
}

fn match_token(toks: &Vec<Tok>, tok: Tok) -> Vec<Tok> {
    match toks.split_first() {
        None => panic!("Invalid Input: {:?}", tok),
        Some((h, t)) if *h == tok => t.to_vec(),
        Some((h, _)) => panic!(
            "Invalid Input: expected {:?}, from input {:?}, got {:?}",
            tok, toks, *h
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer;
    use super::*;

    #[test]
    #[should_panic]
    fn nothing() {
        parser(lexer::lexer(&String::from(""), 0));
    }

    #[test]
    #[should_panic]
    fn invalid_order() {
        parser(lexer::lexer(&String::from("5 + 8 + 9 8"), 0));
    }

    #[test]
    fn add_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("+ 5 4"), 0)),
            Expr::Add((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn add_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("+ 5 + 4 3"), 0)),
            Expr::Add((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Add((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn sub_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("- 5 4"), 0)),
            Expr::Sub((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn sub_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("- 5 - 4 3"), 0)),
            Expr::Sub((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Sub((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn mult_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("* 5 4"), 0)),
            Expr::Mult((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn mult_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("* 5 * 4 3"), 0)),
            Expr::Mult((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Mult((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn div_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("/ 5 4"), 0)),
            Expr::Div((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn div_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("/ 5 / 4 3"), 0)),
            Expr::Div((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Div((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn mixed_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("- 5 / 4 3"), 0)),
            Expr::Sub((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Div((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }
}
