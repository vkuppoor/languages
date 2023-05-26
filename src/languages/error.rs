use core::result;
use std::error::Error as StdError;
use std::fmt;

pub type Result<T, E> = result::Result<T, Error<E>>;

pub struct Error<T> {
    kind: ErrorKind<T>,
}

#[derive(Debug, Clone)]
enum ErrorKind<T> {
    InvalidInput {
        input: T,
    },
    MismatchedToken {
        expected_tok: T,
        tok_list: Vec<T>,
        actual_tok: T,
    },
    TokensEmpty,
    TokensNotEmpty {
        tok_list: Vec<T>,
    },
    ProductionRuleFailure {
        prod_rule: String,
        tok_list: Vec<T>,
    },
}

impl<T: fmt::Debug> StdError for ErrorKind<T> {}

impl<T: fmt::Debug> fmt::Display for ErrorKind<T> {
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
            ErrorKind::TokensNotEmpty { tok_list } => write!(f, "tokens list: {:?}", tok_list),
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

    pub fn mismatched_token(expected_tok: T, tok_list: Vec<T>, actual_tok: T) -> Self {
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

    pub fn tokens_not_empty(tok_list: Vec<T>) -> Self {
        Self {
            kind: ErrorKind::TokensNotEmpty { tok_list },
        }
    }

    pub fn production_rule_failure(prod_rule: String, tok_list: Vec<T>) -> Self {
        Self {
            kind: ErrorKind::ProductionRuleFailure {
                prod_rule,
                tok_list,
            },
        }
    }
}