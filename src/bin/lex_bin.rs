use std::io::{self, Read};

use compier::lexer::{self, LexStep, Lexer, TokenKind};

fn main() {
    let mut src = String::new();
    io::stdin().read_to_string(&mut src).unwrap();

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
