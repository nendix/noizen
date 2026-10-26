# Audio validation log

## Backend prototype

Rodio 0.21.1, playback + WAV only initially; `examples/audio_probe.rs` mixes
quiet 440/660 Hz tones into one output stream. No external player is invoked.

Local run: Apple M1 Pro (8 cores), 16 GiB, macOS Darwin 27 arm64, release build.
Default output was Bluetooth Buds, stereo 44,100 Hz F32. Stream opened and
remained alive for 20 seconds; SIGINT exited with status 0 and dropped the stream.
This verifies backend/device initialization and shutdown, **not audible playback**:
an operator still needs to confirm sound and device release by listening.

Three `ps` samples at 5-second intervals: CPU 0.0%, RSS 17,264 / 17,184 /
17,184 KiB. CPU is below this sampling resolution, not zero cost. Release probe:
799,792 bytes. This is a tone-only baseline, not final product performance.

CPAL 0.16 uses CoreAudio on macOS (system framework). Linux uses ALSA:
build requires pkg-config and ALSA development headers (`libasound2-dev` on
Debian/Ubuntu, `alsa-lib-devel` on Fedora); deployed binaries require libasound
and a usable output device/service. PipeWire/PulseAudio setups may need their
ALSA integration. Bundling Rust code and assets does not bundle OS audio.
Source: https://github.com/RustAudio/cpal/blob/v0.16.0/README.md
Linux runtime/device validation remains pending.

Proposed acceptance budgets, **not yet agreed**: release binary <= 20 MiB,
steady-state RSS <= 100 MiB, CPU <= 5% of one core on this M1 Pro for one sound
and all eight together. Measure final recording decode/startup separately.
No MVP completion claim until budgets and hardware checks are accepted.
