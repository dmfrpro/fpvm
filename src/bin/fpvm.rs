use compiler::vm::bytecode::parse;
use compiler::vm::vm::Vm;
use std::io::Read;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("Failed to read file")
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("Failed to read stdin");
        buf
    };
    let program = match parse(&input) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
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
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            std::process::exit(1);
        }
    }
}
