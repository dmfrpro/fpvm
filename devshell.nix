{
  pkgs,
  inputs,
  perSystem,
  ...
}:

let
  system = pkgs.stdenv.hostPlatform.system;
  pre-commit-check = inputs.git-hooks.lib.${system}.run {
    src = perSystem.flake;
    hooks = {
      rustfmt.enable = true;
      clippy.enable = true;
      end-of-file-fixer.enable = true;
      nixfmt.enable = true;
      markdownlint.enable = true;
    };
  };
in

pkgs.mkShell {
  name = "fpvm-dev";

  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    nixfmt
    markdownlint-cli
    just
  ];

  shellHook = ''
    ${pre-commit-check.shellHook}
    just --list
  '';
}
