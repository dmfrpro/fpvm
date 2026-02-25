use std::io::{self, Read};

use compier::lexer::{Lexer, TokenKind};

fn main() {
    let mut src = String::new();
    io::stdin().read_to_string(&mut src).unwrap();

    let mut lx = Lexer::new(&src);
    loop {
        match lx.next_token() {
            Ok(tok) => {
                println!("{:?} {:?}", tok.kind, tok.span);
                if matches!(tok.kind, TokenKind::Eof) {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Lex error: {:?} at {:?}", e.kind, e.span);
                std::process::exit(1);
            }
        }
    }

    println!("src string: {}",src)
}