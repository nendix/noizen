# noizen

An offline ambient sound mixer for Linux and macOS. Runs in your terminal until
**Ctrl-C**—no downloads, external player, daemon, or TUI.

**MVP candidate:** listening checks, Linux hardware playback, resource-budget
agreement, and published releases remain pending. See [MVP status](docs/mvp.md).

## Install

[Prebuilt releases and macOS Homebrew setup](docs/releases.md) are prepared but
not published yet. For now, build from source.

Requires stable Rust. Linux builds also need `pkg-config libasound2-dev`
(Debian/Ubuntu) or `pkgconf-pkg-config alsa-lib-devel` (Fedora).

```sh
cargo build --release
mkdir -p ~/.local/bin
install -m 755 target/release/noizen ~/.local/bin/noizen
```

Add `~/.local/bin` to your PATH. Linux runtime requires `libasound.so.2` and a
configured ALSA output (possibly PipeWire/PulseAudio ALSA integration).
macOS uses system CoreAudio.

## Use

```sh
noizen sounds
noizen play rain
noizen play rain:70 fireplace:30 --volume 60
noizen play pink:60 green:40
```

Sounds: **white, pink, brown, blue, violet, green, rain, fireplace**.
Green is pink noise shaped around 500 Hz. Levels are integers from 0–100;
both individual levels and overall `--volume` default to **100%**.
**Start with low device volume; 100% is not a safe-loudness guarantee.**

The mix averages selected sources to avoid clipping. Noise colors are
[loudness-balanced](docs/noise-calibration.md); recordings retain their dynamics,
so fireplace is quieter at equal levels.

## Presets

```sh
noizen preset save cozy rain:70 fireplace:30
noizen preset list
noizen preset play cozy --volume 60
noizen preset remove cozy
```

Presets save individual levels, never overall volume. Existing names cannot be
overwritten; remove first. Names allow 1–64 ASCII letters, digits, `-` and `_`.

Stored under `$XDG_CONFIG_HOME/noizen/presets` (default `~/.config/noizen/presets`)
on Linux, or `~/Library/Application Support/noizen/presets` on macOS.

## More

- [Bash, Zsh and Fish completions](docs/completions.md)
- [Recording sources, licenses and attribution](assets/README.md)—retain these
  notices and the [CC BY license](assets/CC-BY-4.0.txt) when redistributing.
- [Development checks](AGENTS.md) · [Validation evidence](docs/audio-validation.md)
