use crate::pipeline::stage::StageOutput;

use crate::semantics::SemanticAnalyzer;
use crate::syntax::node::Node;

pub fn semantic_stage(ast: Node) -> StageOutput<Node> {
    println!("AST:\n{}", ast);
    let analyzer = SemanticAnalyzer::new();
    let sem_errors = analyzer.analyze(&ast);
    if sem_errors.is_empty() {
        println!("Semantic analysis passed.");
    } else {
        eprintln!("Semantic errors:");
        for err in sem_errors {
            eprintln!("  {:?}", err);
        }
    }

    StageOutput::ok(ast)
}
