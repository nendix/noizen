# Shell completions

Generate static Bash, Zsh or Fish scripts with `noizen completions SHELL`.
Generation stays offline and does not initialize audio.

## Bash

```sh
noizen completions bash > ~/.noizen.bash
```

Add `source ~/.noizen.bash` to `~/.bashrc`.

## Zsh

```sh
mkdir -p ~/.zsh/completions
noizen completions zsh > ~/.zsh/completions/_noizen
```

Add the directory to `fpath` **before** `compinit` in `~/.zshrc`:

```zsh
fpath=(~/.zsh/completions $fpath)
autoload -Uz compinit && compinit
```

## Fish

```sh
mkdir -p ~/.config/fish/completions
noizen completions fish > ~/.config/fish/completions/noizen.fish
```

Restart the shell after installation. Commands, flags and supported shell values
have static completion. Dynamic sound/preset suggestions are deferred: runtime
preset enumeration would require shell-specific scripts or an experimental
completion integration.
