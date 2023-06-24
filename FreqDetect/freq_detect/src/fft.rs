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

    pub fn freq_from_fft(&mut self, signal: &[isize], sampling_rate: usize) -> f32 {
        // Create input data vector
        let mut indata: Vec<f64> = Vec::new();
        for (i, value) in signal.iter().enumerate() {
            indata.push((*value as f64 * self.window[i]) as f64);
        }

        // forward transform the signal
        self.r2c.process(&mut indata, &mut self.spectrum).unwrap();

        // Find the index of the highest value in the spectrum
        let mut max_value: usize = 0;
        let mut max_index: usize = 0;
        let abs_array: Vec<usize> = self
            .spectrum
            .iter()
            .enumerate()
            .map(|(i, element)| {
                let norm = element.norm() as usize;
                if norm > max_value {
                    max_value = norm;
                    max_index = i;
                }
                norm
            })
            .collect();

        // Make a parabolic interpolation for obtaining a real value
        let mut true_i: f32 = max_index as f32;
        // Verify that we have one value before and after the maximum index of the spectrum
        if max_index > 0 && max_index < self.length - 1 {
            let prev_idx: usize = max_index.checked_sub(1).unwrap_or(0);
            let next_idx: usize = max_index.checked_add(1).unwrap_or(self.length - 1);

            let prev_value: f32 = (abs_array[prev_idx] as f32).log10();
            let next_value: f32 = (abs_array[next_idx] as f32).log10();
            let current_value: f32 = (abs_array[max_index] as f32).log10();

            let numerator: f32 = 0.5 * (prev_value - next_value);
            let denominator: f32 = prev_value - 2.0 * current_value + next_value;

            if denominator != 0.0 {
                true_i = numerator / denominator + max_index as f32;
            }
        }
        sampling_rate as f32 * true_i / self.length as f32
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
