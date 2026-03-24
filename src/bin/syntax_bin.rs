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

    // match lx.collect_tokens() {
    //     Ok(tokens) => {
    //         let parse_result = parse_syntax(tokens);

    //         match parse_result {
    //             Ok(node) => {
    //                 println!("Successful parse:\n{}", node);
    //             }
    //             Err(syntax_error) => {
    //                 eprintln!("{}", syntax_error);
    //             }
    //         }
            
    //     }

    //     Err(e) => {
    //         eprintln!("lex error: {:?}", e)
    //     }
    // }

