# noizen

An offline ambient sound mixer for Linux and macOS.

## Installation

### Homebrew (macOS)

```sh
brew install nendix/tap/noizen
```

### From source

Requires stable Rust. On Linux, install `pkg-config` and `libasound2-dev`
(Debian/Ubuntu) or `alsa-lib-devel` (Fedora) first.

```sh
cargo install --git https://github.com/nendix/noizen --locked
```

Linux requires ALSA libraries and configured audio output at runtime; macOS uses
system CoreAudio.

## Usage

```sh
noizen sounds
noizen play rain
noizen play rain:70 fireplace:30 --volume 60
noizen play pink:60 green:40
```

## Presets

```sh
noizen preset save cozy rain:70 fireplace:30
noizen preset list
noizen preset play cozy --volume 60
noizen preset remove cozy
```

## Licensing

noizen's source code is licensed under [MIT](LICENSE). Recordings retain their
separate licenses.

Rain is **CC BY 4.0**; fireplace is **Public Domain**. See
[recording sources and attribution](assets/README.md). Redistributed recordings
must retain the applicable attribution and [license notice](assets/CC-BY-4.0.txt).

[Completions](docs/completions.md) · [Releases](docs/releases.md) ·
[Noise calibration](docs/noise-calibration.md)
