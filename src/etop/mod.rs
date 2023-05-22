use std::io::{stdin, stdout, Write};

pub fn etop(language: &String) {
    println!("language chosen: {}", language);
    loop {
        print!("etop # ");
        stdout().flush().expect("flush error");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let script = input.trim();

        println!("- : {}", script);
    }
}
