use realfft::{RealFftPlanner, RealToComplex};
use rustfft::num_complex::Complex;

pub struct FFTProcessor {
    r2c: std::sync::Arc<dyn RealToComplex<f64>>,
    spectrum: Vec<Complex<f64>>,
    window: Vec<f64>,
    length: usize,
}

impl FFTProcessor {
    pub fn new(length: usize) -> Self {
        // make a planner
        let mut real_planner = RealFftPlanner::<f64>::new();
        let r2c: std::sync::Arc<dyn RealToComplex<f64>> = real_planner.plan_fft_forward(length);
        // make a vector for storing the spectrum
        let spectrum = r2c.make_output_vec();
        let window: Vec<f64> = blackman_harris_window(length);
        FFTProcessor {
            r2c,
            spectrum,
            window,
            length,
        }
    }

    pub fn freq_from_fft(&mut self, signal: &[isize], sampling_rate: usize) -> f64 {
        let mut indata = signal
            .iter()
            .enumerate()
            .map(|(i, &v)| v as f64 * self.window[i])
            .collect::<Vec<_>>();

        self.r2c.process(&mut indata, &mut self.spectrum).unwrap();

        let mut max_idx = 0;
        let mut max_value = 0.0;
        for (i, element) in self.spectrum.iter().enumerate() {
            let norm = element.norm();
            if norm > max_value {
                max_value = norm;
                max_idx = i;
            }
        }

        let score = if max_idx > 0 && max_idx < self.length - 1 {
            let prev = self.spectrum[max_idx - 1].norm().log10();
            let curr = max_value.log10();
            let next = self.spectrum[max_idx + 1].norm().log10();

            0.5 * ((prev - next) / (prev - 2.0 * curr + next)) + max_idx as f64
        } else {
            max_idx as f64
        };

        sampling_rate as f64 * score / self.length as f64
    }
}

// Computing the BLACKMAN-HARRIS WINDOW for any given length
fn blackman_harris_window(length: usize) -> Vec<f64> {
    let mut window: Vec<f64> = vec![0.0; length];
    let length_minus_1 = length - 1;

    for n in 0..length {
        let n_f64 = n as f64;
        window[n] = 0.35875
            - 0.48829 * (2.0 * std::f64::consts::PI * n_f64 / length_minus_1 as f64).cos()
            + 0.14128 * (4.0 * std::f64::consts::PI * n_f64 / length_minus_1 as f64).cos()
            - 0.01168 * (6.0 * std::f64::consts::PI * n_f64 / length_minus_1 as f64).cos();
    }

    window
}
