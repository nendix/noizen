# noizen

An offline ambient sound mixer for Linux and macOS. Playback stays in the
terminal until **Ctrl-C**, with no downloads or external player.

## Usage

```sh
noizen sounds
noizen play rain
noizen play rain:70 fireplace:30 --volume 60
noizen play pink:60 green:40
```

Sounds: **white, pink, brown, blue, violet, green, rain, fireplace**.
Green is pink noise shaped around 500 Hz. Levels range from 0–100; individual
levels and overall `--volume` default to **100%**. Start with low device volume.
The mix averages sources to prevent clipping; fireplace is naturally quieter.

## Presets

```sh
noizen preset save cozy rain:70 fireplace:30
noizen preset list
noizen preset play cozy --volume 60
noizen preset remove cozy
```

Presets store individual levels, never overall volume. Names allow 1–64 ASCII
letters, digits, `-` and `_`. Existing presets cannot be overwritten.

Default directory: `~/.config/noizen/presets` on both platforms. Linux also
respects `$XDG_CONFIG_HOME`. Existing macOS presets in
`~/Library/Application Support/noizen/presets` must be copied to the new directory;
noizen does not move or delete them automatically.

## Build

With stable Rust: `cargo build --release`.
Linux builds require `pkg-config` and ALSA headers (`libasound2-dev` on
Debian/Ubuntu, `alsa-lib-devel` on Fedora). Runtime needs `libasound.so.2` and
configured audio output; macOS uses system CoreAudio.

## Licensing

No source-code license has been declared for noizen. The recording licenses do
not grant a license to the application itself.

Rain is **CC BY 4.0**; fireplace is **Public Domain**. See
[recording sources and attribution](assets/README.md). Redistributed recordings
must retain the applicable attribution and [license notice](assets/CC-BY-4.0.txt).

[Completions](docs/completions.md) · [Releases](docs/releases.md) ·
[Noise calibration](docs/noise-calibration.md)
