# Working on noizen

## Product

Build a lightweight foreground ambient sound CLI for Linux and macOS. Read
`docs/mvp.md` for the agreed scope and acceptance criteria. This repository is
currently a scaffold, not a working audio player; keep that distinction clear.

## Architecture and scope

- Rust with clap for parsing and clap_complete for Bash, Zsh, and Fish completions.
- Keep CLI parsing, audio playback, and preset persistence separate as they are implemented.
- Evaluate rodio before committing to an audio backend. Use one output stream for mixing.
- Generate white, pink, brown, blue, violet, and green noise locally.
- Green means pink noise shaped around 500 Hz; document and test this interpretation.
- Embed licensed, compressed rain and fireplace recordings in the binary. No runtime downloads or external player/codec commands.
- Presets store sound IDs and individual levels, never overall volume. Use TOML in the platform configuration directory.
- No duration flag, daemon, TUI, accounts, network service, or plugin framework.
- Dynamic sound/preset completion is conditional: include only if small and maintainable across all three shells, without network or audio initialization.

## Implementation

Prefer the smallest complete change and existing patterns. Do not add placeholder
success paths for unimplemented features. Add modules when real responsibilities
need them, not speculative layers. Do not add dependencies without a concrete use.

Validate IDs and levels (0–100) at the CLI/storage boundary. Preserve existing
presets on failed writes; never silently overwrite a named preset. Keep audio
output bounded, avoid clipping when mixing, and avoid allocations or blocking I/O
in the real-time audio path. Verify filters remain stable over long playback.

Record asset sources and redistribution licenses before committing audio files.
Do not assume a single binary eliminates Linux system audio-library requirements.
Never commit credentials, personal presets, build output, or unlicensed recordings.

## Verification

Run before committing:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Add focused tests for nontrivial logic. Audio work also needs spectral/stability
checks and real-device listening for loops, mixing, and shutdown. Report which
platforms were actually tested; compilation is not playback verification. Measure
CPU, memory, and release binary size before claiming the app is lightweight.

Keep README status and MVP checkboxes honest. Stop at the requested scope.
