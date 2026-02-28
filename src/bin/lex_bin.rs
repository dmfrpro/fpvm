use std::io::{self, Read};

use compier::lexer::{Lexer};

fn main() {
    let mut src = String::new();

    match io::stdin().read_to_string(&mut src) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to read stdin: {}", e);
            std::process::exit(1)
        }
    }

    println!("src string: {}", src);
    let mut lx = Lexer::new(src);

    match lx.collect_tokens() {
        Ok(tokens) => {
            for tok in tokens {
                println!("{:?} {:?}", tok.kind, tok.span)
            }
        }

        Err(e) => {
            eprintln!("lex error: {:?}", e)
        }
    }
}
