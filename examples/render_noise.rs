//! Development-only mono f32le renderer for offline loudness measurements.
#[path = "../src/noise.rs"]
mod noise;

use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2
        || !["white", "pink", "brown", "blue", "violet", "green"].contains(&args[0].as_str())
    {
        return Err(
            "usage: render_noise COLOR SEED (writes 30 seconds of mono 44100 Hz f32le)".into(),
        );
    }
    let mut generator = noise::Noise::new(&args[0], args[1].parse()?);
    for _ in 0..noise::RATE {
        generator.sample();
    }
    let mut output = BufWriter::new(io::stdout().lock());
    for _ in 0..noise::RATE * 30 {
        output.write_all(&generator.sample().to_le_bytes())?;
    }
    output.flush()?;
    Ok(())
}
