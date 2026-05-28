# FPVM

## Contributing

### Install nix

```shell
curl -fsSL https://install.determinate.systems/nix | sh -s -- install
```

### Enable flakes

```shell
echo "experimental-features = nix-command flakes" > ~/.config/nix/nix.conf
echo "trusted-users = root $USER" >> ~/.config/nix/nix.conf
```

### Install direnv & nix-direnv

```shell
nix profile add "nixpkgs#direnv"
nix profile add "nixpkgs#nix-direnv"
echo 'source $HOME/.nix-profile/share/nix-direnv/direnvrc' > $HOME/.config/direnv/direnvrc

# Bash
echo 'eval "$(direnv hook bash)"' >> ~/.bashrc && source ~/.bashrc

# Zsh
echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc && source ~/.zshrc

# Oh-my-Zsh
omz plugin enable direnv
```

### Activate direnv

```shell
cd fpvm
direnv allow .
```
