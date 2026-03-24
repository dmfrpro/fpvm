use std::io::{self, Read};

use compiler::lexer::Lexer;
use compiler::syntax::parse_syntax;

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
    let (ast, errors) = parse_syntax(lx.collect_tokens());

    match ast {
        Some(node) => {
            println!("Successful parse:\n{}", node);
        }
        None => {
            eprintln!("Empty ast");
        }
    }

    if !errors.is_empty() {
        eprintln!("ERRORS:");
        for err in errors {
            eprintln!("{}", err);
        }
    }
}
