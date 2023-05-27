use super::Expr;

pub fn interpreter(ast: Expr) -> i32 {
    match ast {
        Expr::Add((e1, e2)) => {
            let e1 = interpreter(*e1);
            let e2 = interpreter(*e2);
            e1 + e2
        }
        Expr::Sub((e1, e2)) => {
            let e1 = interpreter(*e1);
            let e2 = interpreter(*e2);
            e1 - e2
        }
        Expr::Mult((e1, e2)) => {
            let e1 = interpreter(*e1);
            let e2 = interpreter(*e2);
            e1 * e2
        }
        Expr::Div((e1, e2)) => {
            let e1 = interpreter(*e1);
            let e2 = interpreter(*e2);
            e1 / e2
        }
        Expr::Int(i) => i,
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer;
    use super::super::parser;
    use super::*;

    #[test]
    fn add_basic() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("+ 5 4"), 0)).unwrap()),
            9
        )
    }

    #[test]
    fn add_basic_nested() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("+ 5 + 4 3"), 0)).unwrap()),
            12
        )
    }

    #[test]
    fn sub_basic() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("- 5 4"), 0)).unwrap()),
            1
        )
    }

    #[test]
    fn sub_basic_nested() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("- 5 - 4 3"), 0)).unwrap()),
            4
        )
    }

    #[test]
    fn mult_basic() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("* 5 4"), 0)).unwrap()),
            20
        )
    }

    #[test]
    fn mult_basic_nested() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("* 5 * 4 3"), 0)).unwrap()),
            60
        )
    }

    #[test]
    fn div_basic() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("/ 5 4"), 0)).unwrap()),
            1
        )
    }

    #[test]
    fn div_basic_nested() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("/ 20 / 12 3"), 0)).unwrap()),
            5
        )
    }

    #[test]
    fn mixed_basic_nested() {
        assert_eq!(
            interpreter(parser::parser(lexer::lexer(&String::from("- 10 / 12 3"), 0)).unwrap()),
            6
        )
    }
}
