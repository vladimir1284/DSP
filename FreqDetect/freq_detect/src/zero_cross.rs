const SIGNAL_SIZE: usize = 512;

pub fn freq_from_crossings(signal: [isize; SIGNAL_SIZE], sampling_rate: usize) -> f32 {
    // Find all indices right before a rising-edge zero crossing
    let indices: Vec<usize> = (0..SIGNAL_SIZE - 2)
        // SIGNAL_SIZE - 2 instead of -1 for obtaing the same resutls as in python
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
