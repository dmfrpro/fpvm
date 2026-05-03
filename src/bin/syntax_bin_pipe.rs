use compiler::pipeline::pipeline::Pipeline;
use compiler::stages::lexer_stage;
use compiler::stages::read_from_stdin;
use compiler::stages::semantic_stage;
use compiler::stages::syntax_stage;

fn main() {
    let pipeline = Pipeline::new(read_from_stdin)
        .then(lexer_stage)
        .then(syntax_stage)
        .then(semantic_stage);

    let output = pipeline.run(());

    match output.value {
        Some(value) => {
            println!("AST:\n{}", value);
        }
        None => {
            eprintln!("syntax stage error!");
            std::process::exit(1)
        }
    }
}
