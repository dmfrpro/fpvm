{ pkgs, ... }:

let
  syntaxTest = builtins.readFile ../tests/syntax_test.rs;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "fpvm-syntax-test";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  TEST_HASH = builtins.hashString "sha256" syntaxTest;

  doCheck = true;

  checkPhase = ''
    cargo test --test syntax_test
  '';

  installPhase = ''
    touch $out
  '';
}
