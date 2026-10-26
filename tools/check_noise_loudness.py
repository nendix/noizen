#!/usr/bin/env python3
"""Offline regression check; requires Python 3, Cargo and FFmpeg's loudnorm filter."""
import json
import math
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
COLORS = ("white", "pink", "brown", "blue", "violet", "green")
TARGET_LUFS = -30.0


def main():
    subprocess.run(
        ["cargo", "build", "--locked", "--release", "--example", "render_noise",
         "--target-dir", str(ROOT / "target")],
        cwd=ROOT, check=True,
    )
    print("color    seed    mono LUFS    true peak dBTP", flush=True)
    for color in COLORS:
        for seed in (789, 123, 2026):
            audio = subprocess.check_output(
                [str(ROOT / "target/release/examples/render_noise"), color, str(seed)],
                cwd=ROOT,
            )
            assert len(audio) == 30 * 44100 * 4, "unexpected rendered sample count"
            result = subprocess.run(
                ["ffmpeg", "-hide_banner", "-nostats", "-f", "f32le", "-ar", "44100",
                 "-ac", "1", "-i", "pipe:0", "-af",
                 f"loudnorm=I={TARGET_LUFS}:print_format=json", "-f", "null", "-"],
                input=audio, capture_output=True, check=True,
            )
            # FFmpeg may print progress after the flat loudnorm JSON object.
            log = result.stderr.decode()
            stats, _ = json.JSONDecoder().raw_decode(log[log.rindex("{"):])
            loudness = float(stats["input_i"])
            peak = float(stats["input_tp"])
            print(f"{color:8} {seed:4} {loudness:12.2f} {peak:17.2f}", flush=True)
            assert math.isfinite(loudness) and abs(loudness - TARGET_LUFS) <= 0.5, (
                f"{color}/{seed}: {loudness} LUFS outside target ±0.5 LU"
            )
            assert math.isfinite(peak) and peak <= -6.0, (
                f"{color}/{seed}: insufficient true-peak headroom ({peak} dBTP)"
            )


if __name__ == "__main__":
    main()
