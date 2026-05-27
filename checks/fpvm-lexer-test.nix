{ pkgs, ... }:

let
  # builtins.readFile for test files in tests/
  lexerTest = builtins.readFile ../tests/lexer_test.rs;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "fpvm-lexer-test";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  TEST_HASH = builtins.hashString "sha256" lexerTest;

  doCheck = true;

  checkPhase = ''
    cargo test --test lexer_test
  '';

  installPhase = ''
    touch $out
  '';
}
