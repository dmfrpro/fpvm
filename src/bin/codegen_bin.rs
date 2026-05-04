use compiler::pipeline::pipeline::Pipeline;
// use compiler::stages::codegen_stage;
use compiler::stages::lexer_stage;
use compiler::stages::read_from_stdin;
// use compiler::stages::semantic_stage;
use compiler::stages::syntax_stage;
use compiler::pipeline::types::print_diagnostics;
fn main() {
    let pipeline = Pipeline::new(read_from_stdin)
        .then(lexer_stage)
        .then(syntax_stage);
    // .then(semantic_stage);
    // .then(codegen_stage);

    let output = pipeline.run(());

    match (output.value, output.diagnostics.is_empty()) {
        (Some(value), true) => {
            println!("Pipeline successed with value:\n{}", value);
        }
        (Some(value), false) => {
            println!("Pipeline partially successed");
            println!("Value:{}", value);
            print_diagnostics(output.diagnostics);
        }
        (None, _) => {
            eprintln!("Pipeline failed");
            print_diagnostics(output.diagnostics);
            std::process::exit(1)
        }
    }
}
