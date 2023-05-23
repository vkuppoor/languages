use super::{Expr, Tok};

pub fn parser(toks: Vec<Tok>) -> Expr {
    unimplemented!()
}

fn match_token(toks: Vec<Tok>, tok: Tok) -> Vec<Tok> {
    match toks.split_first() {
        None => panic!("match_token failure: {:?}", tok),
        Some((h, t)) if *h == tok => t.to_vec(),
        Some((h, _)) => panic!("expected"),
    }
}
