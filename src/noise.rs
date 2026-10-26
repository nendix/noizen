pub const RATE: u32 = 44_100;

pub struct Noise {
    color: String,
    rng: u64,
    gain: f32,
    pink: [f64; 7],
    brown: f64,
    previous: f64,
    // Direct-form bandpass state (500 Hz, Q = 0.7).
    x: [f64; 2],
    y: [f64; 2],
}

impl Noise {
    pub fn new(color: &str, seed: u64) -> Self {
        Self {
            color: color.into(),
            rng: seed.max(1),
            // Mono -30 LUFS calibration; see docs/noise-calibration.md.
            gain: match color {
                "white" => 0.039084,
                "pink" => 0.190766,
                "brown" => 0.427071,
                "blue" => 0.423156,
                "violet" => 0.054702,
                "green" => 0.452376,
                _ => 1.0,
            },
            pink: [0.0; 7],
            brown: 0.0,
            previous: 0.0,
            x: [0.0; 2],
            y: [0.0; 2],
        }
    }
    pub fn sample(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        let white = (self.rng >> 11) as f64 / ((1u64 << 53) as f64) * 2.0 - 1.0;
        let value = match self.color.as_str() {
            "white" => white,
            "violet" => {
                let v = (white - self.previous) / 2.0;
                self.previous = white;
                v
            }
            "brown" => {
                self.brown = 0.999 * self.brown + 0.02 * white;
                self.brown
            }
            _ => {
                // Paul Kellet's pinking filter; coefficients assume 44.1 kHz.
                let a = [0.99886, 0.99332, 0.969, 0.8665, 0.55, -0.7616];
                let b = [
                    0.0555179, 0.0750759, 0.153852, 0.3104856, 0.5329522, -0.016898,
                ];
                for i in 0..6 {
                    self.pink[i] = a[i] * self.pink[i] + b[i] * white;
                }
                let pink = (self.pink.iter().sum::<f64>() + white * 0.5362) / 10.0;
                self.pink[6] = white * 0.115926;
                match self.color.as_str() {
                    "blue" => {
                        let v = (pink - self.previous) / 2.0;
                        self.previous = pink;
                        v
                    }
                    "green" => {
                        let w = 2.0 * std::f64::consts::PI * 500.0 / RATE as f64;
                        let alpha = w.sin() / 1.4;
                        let v = (alpha * (pink - self.x[1]) + 2.0 * w.cos() * self.y[0]
                            - (1.0 - alpha) * self.y[1])
                            / (1.0 + alpha);
                        self.x = [pink, self.x[0]];
                        self.y = [v, self.y[0]];
                        v
                    }
                    _ => pink,
                }
            }
        };
        value.clamp(-1.0, 1.0) as f32 * self.gain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustfft::{FftPlanner, num_complex::Complex};
    fn power(color: &str) -> Vec<f64> {
        let n = 65536;
        let mut noise = Noise::new(color, 789);
        for _ in 0..RATE {
            noise.sample();
        }
        let mut spectrum = vec![0.0; n / 2];
        let fft = FftPlanner::new().plan_fft_forward(n);
        for _ in 0..8 {
            let mut data: Vec<_> = (0..n)
                .map(|i| {
                    let s = noise.sample();
                    assert!(s.is_finite() && s.abs() <= 1.0);
                    Complex::new(
                        s as f64
                            * (0.5
                                - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos()),
                        0.0,
                    )
                })
                .collect();
            fft.process(&mut data);
            for (p, c) in spectrum.iter_mut().zip(data) {
                *p += c.norm_sqr();
            }
        }
        spectrum
    }
    fn band(p: &[f64], lo: f64, hi: f64) -> f64 {
        let start = (lo * 65536.0 / RATE as f64) as usize;
        let end = (hi * 65536.0 / RATE as f64) as usize;
        p[start..end].iter().sum::<f64>() / (end - start) as f64
    }
    #[test]
    fn approximate_spectra() {
        // Equal-width log-spaced bands, 200–3200 Hz, away from DC/Nyquist.
        for (color, expected) in [
            ("white", 0.0),
            ("pink", -1.0),
            ("brown", -2.0),
            ("blue", 1.0),
            ("violet", 2.0),
        ] {
            let p = power(color);
            let slope = (band(&p, 2800.0, 3200.0) / band(&p, 200.0, 600.0)).log2()
                / (3000.0_f64 / 400.0).log2();
            assert!((slope - expected).abs() < 0.35, "{color}: {slope}");
        }
        let p = power("green");
        assert!(band(&p, 400.0, 600.0) > band(&p, 40.0, 100.0) * 2.0);
        assert!(band(&p, 400.0, 600.0) > band(&p, 4000.0, 6000.0) * 10.0);
    }
    #[test]
    fn calibration_only_attenuates_without_changing_shape() {
        for color in ["white", "pink", "brown", "blue", "violet", "green"] {
            let mut calibrated = Noise::new(color, 123);
            let mut raw = Noise::new(color, 123);
            raw.gain = 1.0;
            assert!(calibrated.gain > 0.0 && calibrated.gain < 1.0);
            for _ in 0..RATE {
                let input = raw.sample();
                let output = calibrated.sample();
                assert_eq!(output, input * calibrated.gain);
                assert!(output.is_finite() && output.abs() <= input.abs());
            }
        }
    }

    #[test]
    fn stable_over_time() {
        for color in ["white", "pink", "brown", "blue", "violet", "green"] {
            let mut n = Noise::new(color, 123);
            let mut energies = Vec::new();
            for _ in 0..12 {
                let mut energy = 0.0;
                for _ in 0..RATE {
                    let v = n.sample();
                    assert!(v.is_finite() && v.abs() <= 1.0);
                    energy += (v as f64).powi(2);
                }
                energies.push(energy);
            }
            let min = energies.iter().copied().fold(f64::INFINITY, f64::min);
            let max = energies.iter().copied().fold(0.0, f64::max);
            assert!(min > 0.0 && max / min < 2.5, "{color}: {energies:?}");
        }
    }
}
