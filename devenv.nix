{ ... }:

{
  languages.rust.enable = true;

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
    end-of-file-fixer.enable = true;
    nixfmt.enable = true;
    markdownlint.enable = true;
  };
}
