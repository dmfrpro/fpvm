use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use compiler::codegen::BytecodeProgram;
use compiler::stages::{codegen_stage, lexer_stage, read_from_file, semantic_stage, syntax_stage};

use compiler::vm::bytecode::parse;
use compiler::vm::vm::Vm;

#[derive(Debug)]
struct Args {
    input_file: PathBuf,
    verbose: bool,
}

fn main() {
    let args = parse_args();

    let bytecode = compile(&args.input_file, args.verbose);

    run_vm(&bytecode.to_string());
}

fn parse_args() -> Args {
    let mut input_file = None;
    let mut verbose = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-v" | "--verbose" => {
                verbose = true;
            }

            "-h" | "--help" => {
                print_usage_and_exit();
            }

            _ => {
                if input_file.is_none() {
                    input_file = Some(PathBuf::from(arg));
                } else {
                    panic!("unexpected argument: {arg}");
                }
            }
        }
    }

    let input_file = input_file.unwrap_or_else(|| {
        print_usage_and_exit();
    });

    Args {
        input_file,
        verbose,
    }
}

fn print_usage_and_exit() -> ! {
    eprintln!("usage: fp-run <input-file> [-v|--verbose]");
    std::process::exit(1);
}

fn compile(input_file: &PathBuf, verbose: bool) -> BytecodeProgram {
    let file_dir = input_file.parent().unwrap_or_else(|| Path::new("."));

    let file_stem = input_file
        .file_stem()
        .unwrap_or_else(|| input_file.as_os_str())
        .to_string_lossy()
        .to_string();

    let source_output = read_from_file(input_file);

    let source = match source_output.value {
        Some(value) => value,
        None => {
            eprintln!("{:#?}", source_output.diagnostics);
            panic!("failed to read source file {}", input_file.display());
        }
    };

    let lexer_output = lexer_stage(source);

    let tokens = match lexer_output.value {
        Some(value) => value,
        None => {
            eprintln!("{:#?}", lexer_output.diagnostics);
            panic!("lexer failed");
        }
    };

    if verbose {
        let output_path = file_dir.join(format!("{file_stem}.lexer"));
        fs::write(&output_path, format!("{tokens:#?}\n")).unwrap_or_else(|e| {
            panic!("cannot write {}: {e}", output_path.display());
        });
    }

    let syntax_output = syntax_stage(tokens);

    let ast = match syntax_output.value {
        Some(value) => value,
        None => {
            eprintln!("{:#?}", syntax_output.diagnostics);
            panic!("syntax analysis failed");
        }
    };

    if verbose {
        let output_path = file_dir.join(format!("{file_stem}.ast"));
        fs::write(&output_path, format!("{ast:#?}\n")).unwrap_or_else(|e| {
            panic!("cannot write {}: {e}", output_path.display());
        });
    }

    let semantic_output = semantic_stage(ast);

    let checked_program = match semantic_output.value {
        Some(value) => value,
        None => {
            eprintln!("{:#?}", semantic_output.diagnostics);
            panic!("semantic analysis failed");
        }
    };

    if verbose {
        let semantic_output_path = file_dir.join(format!("{file_stem}.semantic"));

        fs::write(&semantic_output_path, format!("{checked_program:#?}\n")).unwrap_or_else(|e| {
            panic!("cannot write {}: {e}", semantic_output_path.display());
        });

        let symbol_table_output_path = file_dir.join(format!("{file_stem}.symbol_table"));

        fs::write(
            &symbol_table_output_path,
            format!("{:#?}\n", &checked_program.symbol_table),
        )
        .unwrap_or_else(|e| {
            panic!("cannot write {}: {e}", symbol_table_output_path.display());
        });
    }

    let codegen_output = codegen_stage(checked_program);

    let generated = match codegen_output.value {
        Some(value) => value,
        None => {
            eprintln!("{:#?}", codegen_output.diagnostics);
            panic!("code generation failed");
        }
    };

    if verbose {
        let output_path = file_dir.join(format!("{file_stem}.fpbc"));

        fs::write(&output_path, format!("{}\n", generated.bytecode)).unwrap_or_else(|e| {
            panic!("cannot write {}: {e}", output_path.display());
        });
    }

    generated.bytecode
}

fn run_vm(input: &str) {
    let program = match parse(input) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("Parse error: {}", error);
            std::process::exit(1);
        }
    };

    let mut vm = Vm::new(program);

    match vm.run() {
        Ok(_) => {
            for line in vm.take_output() {
                println!("{}", line);
            }
        }

        Err(error) => {
            eprintln!("Runtime error: {}", error);
            std::process::exit(1);
        }
    }
}
