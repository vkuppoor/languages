pub mod parser {
    use core::result;
    use std::error::Error as StdError;
    use std::fmt;

    pub type Result<E, T, U> = result::Result<E, Error<T, U>>;

    // T reps a languages tokens; U reps a Vec of a language's tokens
    pub struct Error<T, U> {
        kind: ErrorKind<T, U>,
    }

    #[derive(Debug, Clone)]
    enum ErrorKind<T, U> {
        InvalidInput {
            input: T,
        },
        MismatchedToken {
            expected_tok: T,
            tok_list: U,
            actual_tok: T,
        },
        TokensEmpty,
        TokensNotEmpty {
            tok_list: U,
        },
        ProductionRuleFailure {
            prod_rule: String,
            tok_list: U,
        },
    }

    impl<T: fmt::Debug, U: fmt::Debug> StdError for ErrorKind<T, U> {}

    impl<T: fmt::Debug, U: fmt::Debug> fmt::Display for ErrorKind<T, U> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorKind::InvalidInput { input } => write!(f, "invalid input: {:?}", input),
                ErrorKind::MismatchedToken {
                    expected_tok,
                    tok_list,
                    actual_tok,
                } => write!(
                    f,
                    "expected {:?} from {:?} but got {:?}",
                    expected_tok, tok_list, actual_tok
                ),
                ErrorKind::TokensEmpty => write!(f, "token list is empty"),
                ErrorKind::TokensNotEmpty { tok_list } => {
                    write!(f, "tokens list is not empty: {:?}", tok_list)
                }
                ErrorKind::ProductionRuleFailure {
                    prod_rule,
                    tok_list,
                } => write!(
                    f,
                    "production rule failure: [{}]; tokens list: {:?}",
                    prod_rule, tok_list
                ),
            }
        }
    }

    impl<T: fmt::Debug, U: fmt::Debug> fmt::Debug for Error<T, U> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.kind)
        }
    }

    impl<T: fmt::Debug + Clone, U: fmt::Debug + Clone> Error<T, U> {
        pub fn invalid_input(input: T) -> Self {
            Self {
                kind: ErrorKind::InvalidInput { input },
            }
        }

        pub fn mismatched_token(expected_tok: T, tok_list: U, actual_tok: T) -> Self {
            Self {
                kind: ErrorKind::MismatchedToken {
                    expected_tok,
                    tok_list,
                    actual_tok,
                },
            }
        }

        pub fn tokens_empty() -> Self {
            Self {
                kind: ErrorKind::TokensEmpty,
            }
        }

        pub fn tokens_not_empty(tok_list: U) -> Self {
            Self {
                kind: ErrorKind::TokensNotEmpty { tok_list },
            }
        }

        pub fn production_rule_failure(prod_rule: String, tok_list: U) -> Self {
            Self {
                kind: ErrorKind::ProductionRuleFailure {
                    prod_rule,
                    tok_list,
                },
            }
        }
    }
}

pub mod lexer {
    use core::result;
    use std::error::Error as StdError;
    use std::fmt;

    pub type Result<E, T> = result::Result<E, Error<T>>;

    // T reps a languages tokens; U reps a Vec of a language's tokens
    pub struct Error<T> {
        kind: ErrorKind<T>,
    }

    #[derive(Debug, Clone)]
    enum ErrorKind<T> {
        InvalidInput { input: T },
    }

    impl<T: fmt::Debug> StdError for ErrorKind<T> {}

    impl<T: fmt::Debug> fmt::Display for ErrorKind<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorKind::InvalidInput { input } => write!(f, "invalid input: {:?}", input),
            }
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Error<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.kind)
        }
    }

    impl<T: fmt::Debug + Clone> Error<T> {
        pub fn invalid_input(input: T) -> Self {
            Self {
                kind: ErrorKind::InvalidInput { input },
            }
        }
    }
}

pub mod interpreter {
    use core::result;
    use std::error::Error as StdError;
    use std::fmt;

    pub type Result<E> = result::Result<E, Error>;

    // T reps a languages tokens; U reps a Vec of a language's tokens
    pub struct Error {
        kind: ErrorKind,
    }

    #[derive(Debug, Clone)]
    enum ErrorKind {
        DivByZero,
        InvalidType(String),
    }

    impl StdError for ErrorKind {}

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorKind::DivByZero => write!(f, "divide by zero"),
                ErrorKind::InvalidType(type_name) => write!(f, "incorrect type: {:?}", type_name),
            }
        }
    }

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.kind)
        }
    }

    impl Error {
        pub fn div_by_zero() -> Self {
            Self {
                kind: ErrorKind::DivByZero,
            }
        }
        pub fn invalid_type(type_name: &str) -> Self {
            Self {
                kind: ErrorKind::InvalidType(type_name.to_string()),
            }
        }
    }
}
