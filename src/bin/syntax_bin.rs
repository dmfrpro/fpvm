use std::io::{self, Read};

use compier::lexer::Lexer;
use compier::grammar;

fn main() {
    let mut src = String::new();
    io::stdin().read_to_string(&mut src).unwrap();

    println!("src string: {}", src);
    let mut lx = Lexer::new(src);

    match lx.collect_tokens() {
        Ok(tokens) => {
            let token_iter = tokens.into_iter().map(|tok| (tok.span.start, tok.kind, tok.span.end));

            let parser = grammar::ProgramParser::new();
            match parser.parse(token_iter) {
                Ok(inner_result) => match inner_result {
                    Ok(node) => println!("Parsed successfully: {}", node),
                    Err(semantic_error) => println!("Semantic error: {:?}", semantic_error),
                },
                Err(parse_error) => println!("Syntax error: {:?}", parse_error),
            }
        }

        Err(e) => {
            eprintln!("lex error: {:?}", e)
        }
    }
}
