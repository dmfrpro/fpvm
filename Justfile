# Show this help message
help:
    just --list

# Build compiler
build:
    nix build .#fpvm

# Run tests
test:
    nix flake check
