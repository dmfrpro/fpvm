use compiler::tooling::dump::{
    parse_args,
    run_and_write,
};

fn main() {
    let config = parse_args();

    run_and_write(&config);
}