pub struct ZeroCrossProcessor {
    length: usize,
}

impl ZeroCrossProcessor {
    pub fn new(length: usize) -> Self {
        ZeroCrossProcessor { length }
    }

    pub fn freq_from_crossings(&self, signal: &[isize], sampling_rate: usize) -> f32 {
        let indices: Vec<usize> = (0..self.length - 1)
            .filter(|&i| signal[i] < 0 && signal[i + 1] >= 0)
            .collect();

        let crossings: Vec<f32> = indices
            .iter()
            .map(|&i| i as f32 - signal[i] as f32 / (signal[i + 1] - signal[i]) as f32)
            .collect();

        let mean_diff =
            crossings.windows(2).map(|w| w[1] - w[0]).sum::<f32>() / (crossings.len() - 1) as f32;

        sampling_rate as f32 / mean_diff
    }
}
