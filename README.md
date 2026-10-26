# noizen

A lightweight, offline foreground ambient sound mixer for Linux and macOS.

**Status: implemented MVP candidate, not acceptance-complete.** Local macOS
backend runs and Linux container checks are recorded in
[the validation log](docs/audio-validation.md). Audible playback/loop listening,
Linux hardware playback, agreed resource budgets, and published releases remain
pending. See [MVP acceptance](docs/mvp.md).

## Build and run

Install current stable Rust. On Debian/Ubuntu install `pkg-config libasound2-dev`;
on Fedora install `pkgconf-pkg-config alsa-lib-devel`. Then:

```sh
cargo build --release
install -m 755 target/release/noizen ~/.local/bin/noizen
noizen sounds
noizen play rain
noizen play rain:70 fireplace:30 --volume 80
noizen play pink:60 green:40
```

Create `~/.local/bin` first if needed and put it on your PATH. There are no
published release binaries yet. Binary redistributors must retain the
[recording notices](assets/README.md) and [CC BY license](assets/CC-BY-4.0.txt).
Attribution is also embedded in `noizen --help`.

Playback stays in the current terminal until **Ctrl-C**. Missing devices return
an error. No runtime downloads, asset installation, external player or codec
commands are used. Linux still requires `libasound.so.2` (Debian's `libasound2`
or `libasound2t64`) and a configured ALSA output; PipeWire/PulseAudio may require
ALSA integration. macOS uses system CoreAudio frameworks.

## Sounds and levels

Generated: `white`, `pink`, `brown`, `blue`, `violet`, `green`.
Recordings: `rain`, `fireplace`, with [verified licenses and sources](assets/README.md).
Green means **pink noise bandpass-shaped around 500 Hz**, Q 0.7, reducing bass
and high-frequency hiss—not a universal definition of green noise.

Use `SOUND` or `SOUND:LEVEL`, with integer levels 0–100 (default 100). Duplicate
IDs are rejected. The mono mix averages all selected sources, including muted
ones, then applies overall volume; adding sources can reduce existing sources'
loudness. This deliberately reserves headroom, rather than clipping a sum.
Generated noise and decoded recordings are bounded to [-1, 1]. Noise spectra
are tested approximately over 200–3200 Hz at 44.1 kHz; brown rolls off below
about 7 Hz. The six generated colors use fixed attenuation gains targeting
**-30 LUFS mono** at full source/overall levels. This approximately matches their
loudness, not their spectra; recordings are unchanged. See
[calibration measurements and the offline regression check](docs/noise-calibration.md).
Recordings decode once into memory and use a 100 ms loop crossfade.

Overall `--volume` defaults to **100%**. Lower it to keep noizen quieter than
other applications without changing system volume.
This is not a safe acoustic loudness guarantee. Start low and check device volume.
There is no duration flag, daemon, background service, or TUI.

## Presets

```sh
noizen preset save cozy rain:70 fireplace:30
noizen preset list
noizen preset play cozy --volume 60
noizen preset remove cozy
```

Presets store sound IDs and individual levels, **never overall volume**. Labels
are 1–64 ASCII letters/digits/hyphens/underscores. Save refuses existing labels;
remove explicitly before saving a replacement. Writes use a same-directory
temporary file, sync, and atomic no-clobber publication. Invalid persisted TOML,
unknown fields/IDs, duplicate IDs, empty blends, and invalid levels are errors.
Listing also reports corrupt presets rather than silently hiding them; remove
can delete a corrupt preset.

Locations (platform-standard configuration directory plus `presets/`):
- Linux: `$XDG_CONFIG_HOME/noizen/presets`, default `~/.config/noizen/presets`.
- macOS: `~/Library/Application Support/noizen/presets`.

Example `cozy.toml`:

```toml
[[sounds]]
id = "rain"
level = 70

[[sounds]]
id = "fireplace"
level = 30
```

## Shell completions

Scripts are generated offline from clap; these commands never initialize audio.

```sh
# Bash: put the generated file in your bash-completion directory, or source it.
noizen completions bash > ~/.noizen.bash
# Add to ~/.bashrc:
source ~/.noizen.bash

# Zsh: create this directory and add it to fpath BEFORE compinit in ~/.zshrc.
mkdir -p ~/.zsh/completions
noizen completions zsh > ~/.zsh/completions/_noizen
# ~/.zshrc:
fpath=(~/.zsh/completions $fpath)
autoload -Uz compinit && compinit

# Fish:
mkdir -p ~/.config/fish/completions
noizen completions fish > ~/.config/fish/completions/noizen.fish
```

Restart the shell after installation. Commands, flags, and supported shell values
have static completion. Dynamic sound/preset suggestions are deferred: clap's
static generation does not provide runtime preset enumeration consistently in
all three shells without custom shell-specific scripts or an experimental
completion integration. Neither is needed for this MVP.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

CI is configured for Linux/macOS; it does not prove hardware playback. The
[developer audio probe](examples/audio_probe.rs) plays quiet mixed tones until
Ctrl-C. See [AGENTS.md](AGENTS.md) for repository rules.
