use std::io::{self, Read};

use compiler::lexer::Lexer;
use compiler::syntax::parse_syntax;

fn main() {
    let mut src = String::new();
    io::stdin().read_to_string(&mut src).unwrap();

    println!("src string: {}", src);
    let mut lx = Lexer::new(src);

    match lx.collect_tokens() {
        Ok(tokens) => {
            let parse_result = parse_syntax(tokens);

            match parse_result {
                Ok(node) => {
                    println!("Successful parse: {}", node);
                }
                Err(syntax_error) => {
                    eprintln!("{}", syntax_error);
                }
            }
            
        }

        Err(e) => {
            eprintln!("lex error: {:?}", e)
        }
    }
}
