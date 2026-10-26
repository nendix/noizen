# noizen: idea and MVP

## Idea

A lightweight ambient sound CLI for Linux and macOS. Mix natural ambience and
colored noise for focus or relaxation, and save blends under memorable preset
labels. Playback stays in the current terminal until Ctrl-C.

Distribute one executable per supported platform/architecture. Embed compressed
recordings and generate noise locally: no runtime network, asset installation,
external audio player, or external codec command. OS audio services/libraries may
still be required and must be documented.

## Agreed architecture

- Rust; clap command parsing; clap_complete shell scripts.
- Evaluate rodio with a focused audio prototype before selecting the backend.
- One output stream mixing recordings and continuous noise generators.
- Embedded, redistributable rain and fireplace recordings, with seamless loops.
- Generated white, pink, brown, blue, violet, and green noise.
- Green is our documented interpretation: pink noise shaped to emphasize the
  region around 500 Hz, with reduced bass and high-frequency hiss.
- TOML presets in the platform-standard configuration directory, storing sound
  IDs and individual levels only. Overall volume is always a playback option.
- Static completions for Bash, Zsh, and Fish. Dynamic sound IDs and local preset
  names only if a small, maintainable cross-shell implementation is available.

Reference: https://en.wikipedia.org/wiki/Colors_of_noise

## Intended CLI (not all implemented)

```sh
noizen sounds
noizen play rain
noizen play rain:70 fireplace:30 --volume 80
noizen preset save cozy rain:70 fireplace:30
noizen preset play cozy --volume 60
noizen preset list
noizen preset remove cozy
noizen completions bash
noizen completions zsh
noizen completions fish
```

Levels are integers from 0 to 100; omitted source levels default to 100.
Proposed overall-volume default: 50%, pending listening checks. This setting is
not a guarantee of safe acoustic loudness; device volume still matters.
Preset labels must not be usable to escape the configuration directory. Existing
presets must not be silently overwritten.

## MVP acceptance criteria

### CLI and playback

- [ ] Play each of the eight sound IDs individually and in arbitrary blends.
- [x] Validate unknown IDs, malformed blends, and out-of-range levels with useful errors and nonzero exits.
- [x] Per-source levels and overall volume work without clipping the final mix (tested numerically; listening pending).
- [ ] Stay in the foreground; Ctrl-C stops playback and releases the audio device.
- [x] Missing/unavailable audio devices produce useful errors, not panics (Linux container with no output).
- [x] Each noise generator has bounded, stable output and a tested approximate target spectrum over a documented frequency range.
- [ ] Recordings loop without audible clicks or gaps; listen on actual hardware.

### Assets and distribution

- [x] Bundle rain and fireplace; all playback works offline on first launch (embedded decode tested; audible verification pending).
- [x] Record sources, licenses, and required attribution for every recording.
- [x] No external player or codec executable required at runtime.
- [x] Build and test on Linux and macOS; document audio-library/runtime requirements (Linux arm64 container, macOS arm64 host; Linux hardware playback pending).
- [ ] Publish release binaries with platform and architecture labels.
- [ ] Measure release size, steady-state CPU, and memory for individual sounds and a blend; document hardware and results. Agree numerical budgets after the prototype, before declaring MVP complete.

### Presets

- [x] Save, list, play, and remove named blends, surviving application restarts (persistence tested; preset playback shares the mixer).
- [x] Store only sound IDs and individual levels, not overall volume.
- [x] Use the platform-standard configuration directory.
- [x] Reject invalid persisted content with useful errors; failed writes preserve existing data.
- [x] Prevent accidental overwrite and path traversal via preset labels.

### Completions and quality

- [x] Generate static scripts for Bash, Zsh, and Fish from clap definitions.
- [x] Document and smoke-test shell installation and command/flag completion in all three shells (Bash 3.2, Zsh 5.9 on macOS; Fish 4.0.2 in Linux container).
- [x] Time-box dynamic completion assessment; either implement sound/preset suggestions simply across all three shells, or document their deferral. Completion must stay offline and must not initialize audio. Deferred: static clap generation has no common runtime preset lookup; custom per-shell/experimental integration is not justified.
- [x] Scaffold Rust project and focused CLI tests.
- [ ] CI passes formatting, clippy, and tests on Linux and macOS.
- [x] README clearly documents commands, presets, installation, and limitations.

## Non-goals

No duration flag, daemon, background service, TUI, account, telemetry, runtime
downloads, remote catalog, plugin system, grey noise, or black noise.

## Implementation order

1. Scaffold CLI, documentation, CI, Git, and public GitHub repository.
2. Prototype mixed audio and verify dependencies, loops, and resource usage.
3. Implement and test six noise generators and level handling.
4. Acquire licensed recordings and embed seamless loops.
5. Add persistent presets and complete CLI integration.
6. Verify completions, cross-platform playback, packaging, and measured budgets.

## Current status

Implemented MVP candidate, **not acceptance-complete**. Rodio backend, all six
noise generators, bounds-preserving mixing, embedded licensed Vorbis recordings
with crossfaded loops, TOML presets, and static completions are implemented.
Recording rights were verified against original source pages; attribution and
license notices are in `assets/` and embedded help. Tests cover parsing,
persistence/failures, approximate spectra/stability, decode/loop boundaries and
mixing bounds. See [validation evidence](audio-validation.md).

Still required: human listening to each sound/blends and multiple recording loop
boundaries; confirm acoustic shutdown/device release and the provisional 50%
default; Linux hardware playback; agreed numerical performance budgets; remote
CI on this revision and published platform/architecture release binaries.
No push or release publication was performed.
