use crate::etop::etop;

mod etop;

struct Args {
    language: String,
}
fn main() {
    let language = std::env::args().nth(1).expect("no language given");
    let args = Args { language };
    etop(&args.language);
}
