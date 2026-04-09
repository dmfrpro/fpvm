use std::io::{self, Read};

use compiler::lexer::Lexer;
use compiler::semantics::SemanticAnalyzer;
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

    if errors.is_empty() {
        println!("Parse successful!");
    } else {
        eprintln!("Parse failed!");
        for err in errors {
            eprintln!("{}", err);
        }
    }

    match ast {
        Some(node) => {
            println!("AST:\n{}", node);
            let analyzer = SemanticAnalyzer::new();
            let sem_errors = analyzer.analyze(&node);
            if sem_errors.is_empty() {
                println!("Semantic analysis passed.");
            } else {
                eprintln!("Semantic errors:");
                for err in sem_errors {
                    eprintln!("  {:?}", err);
                }
            }
        }
        None => {
            eprintln!("AST not available.");
        }
    }
}
