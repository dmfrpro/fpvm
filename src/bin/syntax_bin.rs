use std::io::{self, Read};

use compier::syntax::Parser;
use compier::lexer::Lexer;

fn main() {
    let mut src = String::new();
    io::stdin().read_to_string(&mut src).unwrap();

    println!("src string: {}", src);
    let mut lx = Lexer::new(src);

    match lx.collect_tokens() {
        Ok(tokens) => {
            match Parser::parse(tokens.into_iter()) {
                Ok(value) => println!("Accepted: {}", value),
                Err(e) => println!("Syntax error: {:?}", e),
            }
        }

        Err(e) => {
            eprintln!("lex error: {:?}", e)
        }
    }
}
