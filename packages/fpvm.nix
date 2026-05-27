{ pkgs, ... }:

pkgs.rustPlatform.buildRustPackage {
  pname = "fpvm";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  doCheck = false;

  meta = {
    description = "FPVM compiler";
    mainProgram = "dump_tool";
  };
}
