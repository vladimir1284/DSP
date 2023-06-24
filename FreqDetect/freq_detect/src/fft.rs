use realfft::{RealFftPlanner, RealToComplex};
use rustfft::num_complex::Complex;

const LENGTH: usize = 511;
pub struct FFTProcessor {
    r2c: std::sync::Arc<dyn RealToComplex<f64>>,
    spectrum: Vec<Complex<f64>>,
    window: [f64; LENGTH],
}

impl FFTProcessor {
    pub fn new() -> Self {
        // make a planner
        let mut real_planner = RealFftPlanner::<f64>::new();
        let r2c: std::sync::Arc<dyn RealToComplex<f64>> = real_planner.plan_fft_forward(LENGTH);
        // make a vector for storing the spectrum
        let spectrum = r2c.make_output_vec();
        let window = blackman_harris_window();
        FFTProcessor {
            r2c,
            spectrum,
            window,
        }
    }

    pub fn freq_from_fft(&mut self, signal: [isize; LENGTH], sampling_rate: usize) -> f32 {
        // Create input data vector
        let mut indata: Vec<f64> = Vec::new();
        for (i, value) in signal.iter().enumerate() {
            indata.push((*value as f64 * self.window[i]) as f64);
        }
        // make a vector for storing the spectrum
        //let mut spectrum: Vec<Complex<f64>> = r2c.make_output_vec();

        // forward transform the signal
        self.r2c.process(&mut indata, &mut self.spectrum).unwrap();

        // Find the index of the highest value in the spectrum
        let mut max_value: usize = 0;
        let mut max_index: usize = 0;
        let mut abs_array: [usize; LENGTH] = [0; LENGTH];
        for (i, element) in self.spectrum.iter().enumerate() {
            abs_array[i] = element.norm() as usize;
            if abs_array[i] > max_value {
                max_value = abs_array[i];
                max_index = i;
            }
        }

        // Make a parabolic interpolation for obtaining a real value
        let mut true_i: f32 = max_index as f32;
        // Verify that we have one value before and after the maximum index of the spectrum
        if max_index > 0 && max_index < LENGTH - 1 {
            let prev_idx: usize = max_index.checked_sub(1).unwrap_or(0);
            let next_idx: usize = max_index.checked_add(1).unwrap_or(LENGTH - 1);

            let prev_value: f32 = (abs_array[prev_idx] as f32).log10();
            let next_value: f32 = (abs_array[next_idx] as f32).log10();
            let current_value: f32 = (abs_array[max_index] as f32).log10();

            let numerator: f32 = 0.5 * (prev_value - next_value);
            let denominator: f32 = prev_value - 2.0 * current_value + next_value;

            if denominator != 0.0 {
                true_i = numerator / denominator + max_index as f32;
            }
        }
        sampling_rate as f32 * true_i / LENGTH as f32
    }
}

fn blackman_harris_window() -> [f64; LENGTH] {
    let mut window: [f64; LENGTH] = [0.0; LENGTH];
    let length_minus_1 = LENGTH - 1;

    for n in 0..LENGTH {
        let n_f64 = n as f64;
        window[n] = 0.35875
            - 0.48829 * (2.0 * std::f64::consts::PI * n_f64 / length_minus_1 as f64).cos()
            + 0.14128 * (4.0 * std::f64::consts::PI * n_f64 / length_minus_1 as f64).cos()
            - 0.01168 * (6.0 * std::f64::consts::PI * n_f64 / length_minus_1 as f64).cos();
    }

    window
}
