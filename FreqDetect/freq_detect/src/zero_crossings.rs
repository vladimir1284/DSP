pub struct ZeroCrossProcessor {
    length: usize,
}

impl ZeroCrossProcessor {
    pub fn new(length: usize) -> Self {
        ZeroCrossProcessor { length }
    }

    pub fn freq_from_crossings(&self, signal: &[isize], sampling_rate: usize) -> f64 {
        let indices: Vec<usize> = (0..self.length - 1)
            .filter(|&i| signal[i] < 0 && signal[i + 1] >= 0)
            .collect();

        let crossings: Vec<f64> = indices
            .iter()
            .map(|&i| i as f64 - signal[i] as f64 / (signal[i + 1] - signal[i]) as f64)
            .collect();

        let mean_diff =
            crossings.windows(2).map(|w| w[1] - w[0]).sum::<f64>() / (crossings.len() - 1) as f64;

        sampling_rate as f64 / mean_diff
    }
}
