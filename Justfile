default:
    just --list

build:
    cargo build

test:
    cargo test

check:
    nix flake check

fmt:
    cargo fmt

clippy:
    cargo clippy

run-lex:
    cargo run --bin lex_bin

run-syntax:
    cargo run --bin syntax_bin

run-codegen:
    cargo run --bin codegen_bin

run-dump INPUT="tests/code/arithmetic.fpvm":
    cargo run --bin dump_tool -- {{INPUT}}
