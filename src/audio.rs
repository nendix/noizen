use crate::{
    noise::{Noise, RATE},
    sound::Sound,
};
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use std::{
    io::Cursor,
    sync::mpsc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

struct Recording {
    samples: Vec<f32>,
    position: usize,
}
impl Recording {
    fn load(bytes: &'static [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut decoder = Decoder::try_from(Cursor::new(bytes))?;
        if decoder.sample_rate() != RATE || decoder.channels() != 2 {
            return Err("embedded recordings must be stereo 44.1 kHz".into());
        }
        // Read samples directly: Vorbis can begin with an empty metadata span.
        let mut samples: Vec<f32> = std::iter::from_fn(|| {
            let left = decoder.next()?;
            let right = decoder.next()?;
            Some((left + right) * 0.5)
        })
        .collect();
        let fade = (RATE / 10) as usize;
        if samples.len() < fade * 2 {
            return Err("recording too short for loop crossfade".into());
        }
        let end = samples.len() - fade;
        // Linear overlap preserves bounds; skip the consumed head on subsequent loops.
        for i in 0..fade {
            let t = i as f32 / fade as f32;
            samples[end + i] = samples[end + i] * (1.0 - t) + samples[i] * t;
        }
        samples.drain(..fade);
        if samples.iter().any(|s| !s.is_finite() || s.abs() > 1.0) {
            return Err("invalid recording samples".into());
        }
        Ok(Self {
            samples,
            position: 0,
        })
    }
    fn sample(&mut self) -> f32 {
        let sample = self.samples[self.position];
        self.position = (self.position + 1) % self.samples.len();
        sample
    }
}

enum Generator {
    Noise(Noise),
    Recording(Recording),
}
struct Mix {
    sources: Vec<(Generator, f32)>,
    volume: f32,
}
impl Mix {
    fn new(sounds: &[Sound], volume: u8) -> Result<Self, Box<dyn std::error::Error>> {
        crate::sound::validate_blend(sounds)?;
        if volume > 100 {
            return Err("volume must be between 0 and 100".into());
        }
        let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64;
        let sources = sounds
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let generator = match s.id.as_str() {
                    "rain" => {
                        Generator::Recording(Recording::load(include_bytes!("../assets/rain.ogg"))?)
                    }
                    "fireplace" => Generator::Recording(Recording::load(include_bytes!(
                        "../assets/fireplace.ogg"
                    ))?),
                    _ => Generator::Noise(Noise::new(&s.id, seed.wrapping_add(i as u64))),
                };
                Ok((generator, s.level as f32 / 100.0))
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
        Ok(Self {
            sources,
            volume: volume as f32 / 100.0,
        })
    }
}
impl Iterator for Mix {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let sum: f32 = self
            .sources
            .iter_mut()
            .map(|(source, level)| {
                let v = match source {
                    Generator::Noise(n) => n.sample(),
                    Generator::Recording(r) => r.sample(),
                };
                v * *level
            })
            .sum();
        Some((sum / self.sources.len() as f32 * self.volume).clamp(-1.0, 1.0))
    }
}
impl Source for Mix {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub fn play(sounds: &[Sound], volume: u8) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;
    let mix = Mix::new(sounds, volume)?;
    let stream = OutputStreamBuilder::open_default_stream().map_err(|e| {
        format!("cannot open default audio output: {e}; check your OS audio device/service")
    })?;
    let sink = Sink::connect_new(stream.mixer());
    sink.append(mix);
    eprintln!("Playing at {volume}% overall volume. Ctrl-C to stop; device volume still matters.");
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(()) => break,
            Err(mpsc::RecvTimeoutError::Timeout) if !sink.empty() => {}
            _ => return Err("audio playback stopped unexpectedly".into()),
        }
    }
    sink.stop();
    drop(stream);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recording_decode_and_loop() {
        for bytes in [
            include_bytes!("../assets/rain.ogg").as_slice(),
            include_bytes!("../assets/fireplace.ogg").as_slice(),
        ] {
            let mut r = Recording::load(bytes).unwrap();
            let first = r.samples[0];
            let last = *r.samples.last().unwrap();
            assert!((first - last).abs() < 0.1, "loop boundary {first} {last}");
            r.position = r.samples.len() - 1;
            assert_eq!(r.sample(), last);
            assert_eq!(r.sample(), first);
        }
    }
    #[test]
    fn mixing_bounds_and_levels() {
        let sounds: Vec<Sound> = crate::sound::IDS
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut mix = Mix::new(&sounds, 80).unwrap();
        for _ in 0..RATE {
            let v = mix.next().unwrap();
            assert!(v.is_finite() && v.abs() <= 0.8);
        }
        let mut silent = Mix::new(&["white:0".parse().unwrap()], 100).unwrap();
        assert_eq!(silent.next(), Some(0.0));
        let mut muted = Mix::new(&["white".parse().unwrap()], 0).unwrap();
        assert_eq!(muted.next(), Some(0.0));
        assert!(Mix::new(&[], 50).is_err());
        assert!(Mix::new(&sounds, 101).is_err());
        let make = |level, volume| Mix {
            sources: vec![(Generator::Noise(Noise::new("white", 42)), level)],
            volume,
        };
        let mut full = make(1.0, 1.0);
        let mut scaled = make(0.7, 0.6);
        for _ in 0..1000 {
            assert!((scaled.next().unwrap() - full.next().unwrap() * 0.7 * 0.6).abs() < 1e-6);
        }
    }
}
