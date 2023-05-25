use std::error::Error as StdError;
use std::fmt;

pub struct Error<T> {
    kind: ErrorKind<T>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind<T> {
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
            ErrorKind::InvalidInput { input } => write!(f, "invali input: {:?}", input),
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
