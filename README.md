# noizen

A lightweight ambient sound CLI for Linux and macOS.

**Status: scaffold.** Help, planned sound listing, and Bash/Zsh/Fish completion
script generation work. Audio playback and presets are not implemented yet.
See [the MVP plan](docs/mvp.md) for scope and acceptance criteria and
[AGENTS.md](AGENTS.md) for contributor guidance.

## Try the scaffold

Install a current stable Rust toolchain, then run:

```sh
cargo run -- --help
cargo run -- sounds
cargo run -- completions bash
cargo run -- completions zsh
cargo run -- completions fish
```

Completion commands print scripts to stdout; they do not modify shell config.
Scripts cover only currently implemented commands. Dynamic sound/preset
completion is conditional, not yet implemented.

```sh
cargo build --release
./target/release/noizen --help
```

## Planned experience

```sh
noizen play rain:70 fireplace:30 --volume 80
noizen preset save cozy rain:70 fireplace:30
noizen preset play cozy --volume 60
```

Six generated noise colors (white, pink, brown, blue, violet, green), plus bundled
rain and fireplace recordings. Fully offline playback, foreground execution,
and named blends that do not save overall volume. Green will be pink noise
shaped around 500 Hz, not a claimed universal definition.

Bundled assets remove downloads, not necessarily OS audio-library requirements.
Audio backend dependencies, recording licenses, and performance are still to be
verified. There is no duration flag, daemon, or TUI.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

CI runs these checks on Linux and macOS. Hardware playback must be tested
separately once implemented.
