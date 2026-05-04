use compiler::pipeline::pipeline::Pipeline;
use compiler::stages::lexer_stage;
use compiler::stages::read_from_stdin;
use compiler::stages::semantic_stage;
use compiler::stages::syntax_stage;
use compiler::stages::codegen_stage;

fn main() {
    let pipeline = Pipeline::new(read_from_stdin)
        .then(lexer_stage)
        .then(syntax_stage)
        .then(semantic_stage);
        // .then(codegen_stage);

    let output = pipeline.run(());


    match (output.value, output.diagnostics.is_empty()) {
        (Some(value), true) => {
            println!("Pipeline successed with value:\n{}", value);
        }
        (Some(value), false) => {
            println!("Pipeline partially successed");
            println!("value{}", value.ast);
            for err in output.diagnostics {
                eprintln!("err:\n{}", err);
            }
        }
        (None, _) => {
            eprintln!("Pipeline failed");
            for err in output.diagnostics {
                eprintln!("err:\n{}", err);
            }
            std::process::exit(1)
        }
    }
}
