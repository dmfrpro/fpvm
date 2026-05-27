{
  inputs,
  system,
  ...
}:

inputs.git-hooks.lib.${system}.run {
  src = ../.;
  hooks = {
    rustfmt.enable = true;
    nixfmt.enable = true;
    markdownlint.enable = true;
  };
}
