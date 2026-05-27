{ pkgs, ... }:

let
  codegenTest = builtins.readFile ../tests/codegen_test.rs;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "fpvm-codegen-test";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  TEST_HASH = builtins.hashString "sha256" codegenTest;

  doCheck = true;

  checkPhase = ''
    cargo test --test codegen_test
  '';

  installPhase = ''
    touch $out
  '';
}
