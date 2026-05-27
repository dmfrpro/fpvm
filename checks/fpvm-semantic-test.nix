{ pkgs, ... }:

let
  semanticTest = builtins.readFile ../tests/semantic_test.rs;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "fpvm-semantic-test";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  TEST_HASH = builtins.hashString "sha256" semanticTest;

  doCheck = true;

  checkPhase = ''
    cargo test --test semantic_test
  '';

  installPhase = ''
    touch $out
  '';
}
