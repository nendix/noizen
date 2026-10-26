# Audio validation log

## Backend decision and prototype (before implementation)

Rodio 0.21.1, initially playback + WAV features; `examples/audio_probe.rs` mixes
quiet 440/660 Hz tones into one output stream. No external player is invoked.
Final application enables **playback + Vorbis only**; unused WAV support removed.

Local host: Apple M1 Pro (8 cores), 16 GiB, Darwin 27 arm64, Rust 1.99.1.
Default output: Bluetooth Buds, stereo 44,100 Hz F32. The release prototype
opened a stream, remained alive 20 seconds, and exited 0 on SIGINT, dropping the
stream. Three `ps` samples: CPU 0.0%, RSS 17,264 / 17,184 / 17,184 KiB.
Release probe: 799,792 bytes. The percentage samples were below reporting
resolution, **not zero CPU usage**. Final measurements below use cumulative CPU
time differences instead.

This verifies backend/device initialization and software shutdown, **not audible
playback**. No person has confirmed sound, loop quality, loudness, or acoustic
shutdown. Those acceptance gates remain open. SIGINT tests exercised the same
handler as terminal Ctrl-C, but were sent programmatically.

## Final application measurements

Same host/output; `cargo build --release`, each sound and all eight together at
`--volume 1`, 20-second runs. Sampled `ps -p PID -o time=,rss=` at roughly 5, 10,
and 15 seconds; SIGINT at 20 seconds. Steady CPU = difference of cumulative CPU
time between first/third sample, divided by the ten-second wall interval.
Rounded CPU time has 0.01-second resolution; scheduling/startup variance applies.
RSS is the largest of the three samples, **not peak allocation**. `wait4` CPU
includes startup/decode and shutdown. No network or external codec/player was
used by the application.

| Sound | CPU time at 5/10/15 s | Steady CPU, % of one core | RSS KiB | Total CPU seconds | SIGINT exit |
| --- | --- | ---: | ---: | ---: | ---: |
| white | .07 / .12 / .16 | 0.9 | 17,776 | .204 | 0 |
| pink | .07 / .12 / .17 | 1.0 | 17,792 | .223 | 0 |
| brown | .07 / .12 / .16 | 0.9 | 17,776 | .206 | 0 |
| blue | .08 / .12 / .17 | 0.9 | 17,792 | .226 | 0 |
| violet | .07 / .11 / .16 | 0.9 | 17,792 | .206 | 0 |
| green | .08 / .14 / .19 | 1.1 | 17,808 | .245 | 0 |
| rain | .25 / .29 / .33 | 0.8 | 45,504 | .372 | 0 |
| fireplace | .10 / .14 / .18 | 0.8 | 25,920 | .222 | 0 |
| all eight | .31 / .38 / .46 | 1.5 | 53,152 | .535 | 0 |

Release executable sizes (unstripped): macOS arm64 **6,517,264 bytes**;
Linux arm64 **6,818,512 bytes**. No published binaries yet.
The full mix sampled about 51.9 MiB RSS. Recordings decode once to mono float
buffers; sample-by-sample downmix avoids allocating a second full stereo buffer.

Proposed budgets, **not yet agreed**: release executable <= 20 MiB,
steady-state RSS <= 100 MiB, CPU <= 5% of one core on this M1 Pro for individual
sounds and all eight together. Current samples meet those proposed limits;
longer runs, other hardware, and audible playback still need verification.

To repeat a measurement, run `target/release/noizen play SOUND --volume 1` in
the background; sample `ps -p "$pid" -o time=,rss=` at 5/10/15 seconds, then
`kill -INT "$pid"; wait "$pid"`. For the blend, pass all eight IDs.
Do not substitute a synthetic/null audio output for a listening test.

## Linux dependencies and validation

CPAL 0.16 uses CoreAudio on macOS (system frameworks). Linux uses ALSA:
build requires pkg-config and ALSA development headers (`libasound2-dev` on
Debian/Ubuntu, `alsa-lib-devel` on Fedora); deployed binaries require libasound
and a usable output device/service. PipeWire/PulseAudio setups may need their
ALSA integration. Bundling Rust code and assets does not bundle OS audio.
Source: https://github.com/RustAudio/cpal/blob/v0.16.0/README.md

Actual Linux checks: Docker, Debian 13 trixie aarch64, Rust 1.99.1; installed
`pkg-config libasound2-dev`, rustfmt and clippy. Formatting, clippy with warnings
denied, all 11 tests, and release build passed. `ldd` reports `libasound.so.2`,
`libgcc_s.so.1`, `libm.so.6`, `libc.so.6`, and the ELF loader. macOS `otool -L`
reports system libraries and CoreAudio/AudioToolbox/Foundation frameworks.

The container has **no audio hardware**. `play white` and preset playback both
exit 1 with `cannot open default audio output ... check your OS audio
device/service` (ALSA also logs device lookup diagnostics); no panic. This tests
the failure path, not Linux playback. No x86_64 build/hardware was tested.

## Tests and shell checks

On macOS and Linux, the 11 tests cover CLI parsing, completion generation,
blend validation, preset persistence/no-clobber, corrupt/unknown TOML fields and
IDs, path traversal, filesystem failures, noise spectra/stability, embedded
recording decode/loop boundary continuity and mix bounds/level scaling.
Spectral tests use deterministic seeds, Hann-windowed 65,536-point FFTs averaged
over eight windows after warmup: approximate white/pink/brown/blue/violet slopes
within 0.35 of 0/-1/-2/+1/+2 over bands centered at 400 and 3000 Hz. Green's
400–600 Hz power exceeds 40–100 Hz and 4–6 kHz bands. Twelve seconds per color
also test bounded/finite output and energy stability. These are approximate
spectral tests, not acoustic calibration.

Linux cross-process smoke: save `cozy`, list in another invocation, attempt
replacement (nonzero, original SHA-256 unchanged), load via preset playback
(reaches expected no-device error), remove, list empty. No audio is initialized
for preset storage/listing or completion generation.

Completion smoke tests:
- Bash 3.2.57 on macOS and Bash in Debian: source generated script, invoke
  `_noizen noizen p noizen` / `_noizen noizen --v play` with matching
  `COMP_WORDS`/`COMP_CWORD`; assert `play` / `--volume` in `COMPREPLY`.
- Zsh 5.9 on macOS: load generated script after `compinit` in an interactive
  pseudo-terminal; `noizen pl<Tab>` becomes `noizen play `, then `--v<Tab>`
  becomes `--volume=`. The pseudo-terminal needed an explicit terminal size.
- Fish 4.0.2 in Debian: source generated script; assert `complete -C 'noizen p'`
  suggests `play` and `complete -C 'noizen play --v'` suggests `--volume`.

Dynamic completion assessment: clap's current static generators do not share a
runtime preset-lookup hook. Per-shell scripts or an experimental dynamic engine
would add maintenance for an optional feature; deferred. No runtime network or
audio initialization is needed by completions.

## Remaining acceptance gates

- Human listening to all eight IDs, blends, level behavior and several loops
  (rain ~124.5 seconds, fireplace ~25.4 seconds after overlap).
- Confirm no audible loop clicks/gaps and acoustic shutdown/device release.
- Accept/change provisional 50% overall default and proposed resource budgets.
- Linux hardware playback/resource measurements; broader architectures.
- Remote CI for these local commits and platform-labelled published releases.

The implementation is an MVP candidate, **not a completed/accepted MVP**.
