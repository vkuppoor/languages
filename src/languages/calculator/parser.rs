use super::super::error::parser::{Error, Result};
use super::{Expr, Tok};

/** Grammar:
E -> + N E | - N E | * N E | / N E | N
N -> -2,147,483,648 | -2,147,483,647 | -2,147,483,646 | ... | 2,147,483,647
*/
pub fn parser(toks: Vec<Tok>) -> Result<Expr, Tok, Vec<Tok>> {
    let (toks, e) = parse_e(toks)?;
    if !toks.is_empty() {
        Err(Error::tokens_not_empty(toks))
    } else {
        Ok(e)
    }
}

fn parse_e(toks: Vec<Tok>) -> Result<(Vec<Tok>, Expr), Tok, Vec<Tok>> {
    if let Some(tok_head) = toks.get(0) {
        match tok_head {
            Tok::TokAdd => {
                let toks = match_token(&toks, Tok::TokAdd)?;
                let (toks, e1) = parse_e(toks)?;
                let (toks, e2) = parse_e(toks)?;
                Ok((toks, Expr::Add((Box::new(e1), Box::new(e2)))))
            }
            Tok::TokSub => {
                let toks = match_token(&toks, Tok::TokSub)?;
                let (toks, e1) = parse_e(toks)?;
                let (toks, e2) = parse_e(toks)?;
                Ok((toks, Expr::Sub((Box::new(e1), Box::new(e2)))))
            }
            Tok::TokMult => {
                let toks = match_token(&toks, Tok::TokMult)?;
                let (toks, e1) = parse_e(toks)?;
                let (toks, e2) = parse_e(toks)?;
                Ok((toks, Expr::Mult((Box::new(e1), Box::new(e2)))))
            }
            Tok::TokDiv => {
                let toks = match_token(&toks, Tok::TokDiv)?;
                let (toks, e1) = parse_e(toks)?;
                let (toks, e2) = parse_e(toks)?;
                Ok((toks, Expr::Div((Box::new(e1), Box::new(e2)))))
            }
            Tok::TokInt(_) => Ok(parse_n(toks)?),
        }
    } else {
        Err(Error::tokens_empty())
    }
}

fn parse_n(toks: Vec<Tok>) -> Result<(Vec<Tok>, Expr), Tok, Vec<Tok>> {
    match toks.get(0).unwrap() {
        Tok::TokInt(i) => {
            let toks = match_token(&toks, Tok::TokInt(*i))?;
            Ok((toks, Expr::Int(*i)))
        }
        _ => Err(Error::production_rule_failure(String::from("N rule"), toks)),
    }
}

fn match_token(toks: &[Tok], tok: Tok) -> Result<Vec<Tok>, Tok, Vec<Tok>> {
    match toks.split_first() {
        None => Err(Error::invalid_input(tok)),
        Some((h, t)) if *h == tok => Ok(t.to_vec()),
        Some((h, _)) => Err(Error::mismatched_token(tok, toks.to_vec(), *h)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer;
    use super::*;

    #[test]
    fn nothing() {
        assert!(parser(lexer::lexer(&String::from(""), 0).unwrap()).is_err());
    }

    #[test]
    fn invalid_order() {
        assert!(parser(lexer::lexer(&String::from("5 + 8 + 9 8"), 0).unwrap()).is_err());
    }

    #[test]
    fn add_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("+ 5 4"), 0).unwrap()).unwrap(),
            Expr::Add((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn add_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("+ 5 + 4 3"), 0).unwrap()).unwrap(),
            Expr::Add((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Add((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn sub_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("- 5 4"), 0).unwrap()).unwrap(),
            Expr::Sub((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn sub_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("- 5 - 4 3"), 0).unwrap()).unwrap(),
            Expr::Sub((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Sub((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn mult_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("* 5 4"), 0).unwrap()).unwrap(),
            Expr::Mult((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn mult_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("* 5 * 4 3"), 0).unwrap()).unwrap(),
            Expr::Mult((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Mult((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn div_basic() {
        assert_eq!(
            parser(lexer::lexer(&String::from("/ 5 4"), 0).unwrap()).unwrap(),
            Expr::Div((Box::new(Expr::Int(5)), Box::new(Expr::Int(4))))
        )
    }

    #[test]
    fn div_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("/ 5 / 4 3"), 0).unwrap()).unwrap(),
            Expr::Div((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Div((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }

    #[test]
    fn mixed_basic_nested() {
        assert_eq!(
            parser(lexer::lexer(&String::from("- 5 / 4 3"), 0).unwrap()).unwrap(),
            Expr::Sub((
                Box::new(Expr::Int(5)),
                Box::new(Expr::Div((Box::new(Expr::Int(4)), Box::new(Expr::Int(3)))))
            ))
        )
    }
}
