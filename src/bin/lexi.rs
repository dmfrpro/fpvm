use compiler::lexer::Lexer;
use std::io::{self, Write};

fn main() {
    let mut lx = Lexer::new("".to_string());
    let stdin = io::stdin();

    loop {
        // input
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("stdout flush error: {e}");
            return;
        }

        // reading line
        let mut line = String::new();
        let bytes = match stdin.read_line(&mut line) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("stdin read error: {e}");
                return;
            }
        };

        // stop condition
        if bytes == 0 {
            return;
        }

        lx.push_line(&line);
        loop {
            match lx.next_token() {
                Ok(Some(tok)) => {
                    println!("{:?} {:?}", tok.kind, tok.span);
                }
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    eprintln!("lex error: {:?}", e);
                }
            }
        }
    }
}
