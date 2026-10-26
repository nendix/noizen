# Generated-noise loudness calibration

The six generated colors use fixed, attenuation-only gains, targeting **-30 LUFS
integrated for mono** at source level 100 and overall volume 100. This reduces
large level differences without adding a limiter, automatic gain control, new
runtime dependency, or audio processing command. Preset levels and the 100%
overall default are unchanged; existing presets containing noise will be quieter.
Recordings are **not** recalibrated by this change.

## Method

Measured offline on macOS arm64 with FFmpeg 9.0.2's `loudnorm` input statistics
(EBU R128 / BS.1770 loudness weighting). No output audio was played.
`examples/render_noise.rs` imports the actual generator and writes 30 seconds of
44.1 kHz mono float samples after a one-second warmup, using deterministic seeds.
FFmpeg analyzes these samples; its normalized output is discarded. We read
`input_i` and `input_tp`, not the filter's output loudness.

Baseline seed 789, before calibration:

| Color | Raw mono LUFS | Fixed linear gain | Attenuation dB |
| --- | ---: | ---: | ---: |
| white | -1.84 | 0.039084 | -28.16 |
| pink | -15.61 | 0.190766 | -14.39 |
| brown | -22.61 | 0.427071 | -7.39 |
| blue | -22.53 | 0.423156 | -7.47 |
| violet | -4.76 | 0.054702 | -25.24 |
| green | -23.11 | 0.452376 | -6.89 |

Gains are `10^((-30 - measured_LUFS) / 20)`, rounded to six decimal places.
The -30 target is below the quietest raw color, so none is boosted. It is a
conservative signal-level choice, **not a safe acoustic loudness threshold**.
Gains apply after the existing sample bound, preserving spectral shape and
bounds while leaving user levels as independent controls. The mono-to-stereo
output/device path may change measured absolute loudness; relative calibration
uses the same mono path for every color.

## Reproduce and regression-check

Install FFmpeg with `loudnorm`, Python 3, and Rust, then run from the repository:

```sh
python3 tools/check_noise_loudness.py
```

This builds a developer renderer, feeds samples directly to FFmpeg, and checks
all six colors at seeds 789, 123, and 2026: integrated loudness within **0.5 LU**
of -30, finite statistics, expected sample count, and true peaks <= **-6 dBTP**.
It exits nonzero on failure. Do not use Python's `-O` option: checks use asserts.
FFmpeg is needed only for this development check, never for normal noizen use.

Observed after calibration (three seeds, 30 seconds each):

| Color | Integrated mono LUFS range | Highest measured true peak dBTP |
| --- | ---: | ---: |
| white | -30.01 to -30.00 | -22.46 |
| pink | -29.99 | -15.52 |
| brown | -30.11 to -30.00 | -7.35 |
| blue | -30.00 to -29.99 | -22.28 |
| violet | -30.01 to -30.00 | -22.22 |
| green | -30.00 to -29.97 | -15.93 |

All 18 measurements passed. Rust tests also check that calibration only
attenuates, is an exact constant scaling of the original waveform, and retains
finite/bounded output, spectral slopes, and temporal stability. Standard checks
pass locally on macOS; Linux and hardware listening were not rerun for this
calibration change. Earlier resource measurements are historical, not new proof.

## Limits

LUFS approximates loudness; it cannot guarantee equal perception for different
spectra, earbud responses, or listeners. A brief listening comparison at a **low,
fixed system volume** remains necessary, including comparisons with rain and
fireplace. Never use maximum system volume as the calibration reference.
True-peak measurements describe these finite test signals, not every possible
seed, hardware reconstruction filter, or safe listening level.
