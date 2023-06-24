use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

mod fft;
mod zero_cross;

const LENGTH: usize = 511;
const SAMPLING_RATE: usize = 100 * 1000 * 1000; // Sampling frequency

fn main() {
    let filename: &str = "TxBurst.csv";
    println!("Reading file \"{}\"\n", filename);

    println!("Computing by zero crossing...");
    compute_zc(filename);
    println!("");
    println!("Computing by FFT...");
    compute_fft(filename);
}

fn compute_fft(filename: &str) {
    let burst: Vec<[isize; LENGTH]> =
        read_signal_from_file(filename).expect("Failed to read signal.");
    let n_signals = burst.len();

    let start_time = Instant::now();

    let mut frequencies = Vec::new();
    for signal in burst {
        // Estimate the frequency
        let frequency: f32 = fft::freq_from_fft(signal, SAMPLING_RATE);
        frequencies.push(frequency);
    }

    let loop_time = start_time.elapsed().as_secs_f32();
    let average_loop_time = loop_time / n_signals as f32;

    println!("Average loop time: {:.2} ms", 1000.0 * average_loop_time);

    if !frequencies.is_empty() {
        let (average_frequency, std_deviation) = ave_std(&frequencies);
        println!("Average frequency: {:.2}", average_frequency);
        println!("Standard deviation of frequencies: {:.2}", std_deviation);
    } else {
        println!("No frequencies computed.");
    }
}
fn compute_zc(filename: &str) {
    let burst: Vec<[isize; LENGTH]> =
        read_signal_from_file(filename).expect("Failed to read signal.");
    let n_signals = burst.len();

    let start_time = Instant::now();

    let mut frequencies = Vec::new();
    for signal in burst {
        // Estimate the frequency
        let frequency: f32 = zero_cross::freq_from_crossings(signal, SAMPLING_RATE);
        frequencies.push(frequency);
    }

    let loop_time = start_time.elapsed().as_secs_f32();
    let average_loop_time = loop_time / n_signals as f32;

    println!("Average loop time: {:.2} ms", 1000.0 * average_loop_time);

    if !frequencies.is_empty() {
        let (average_frequency, std_deviation) = ave_std(&frequencies);
        println!("Average frequency: {:.2}", average_frequency);
        println!("Standard deviation of frequencies: {:.2}", std_deviation);
    } else {
        println!("No frequencies computed.");
    }
}

fn read_signal_from_file(filename: &str) -> Result<Vec<[isize; LENGTH]>, std::io::Error> {
    let file: File = File::open(filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut burst: Vec<[isize; LENGTH]> = Vec::new();
    for line in reader.lines() {
        if let Ok(value) = line {
            let mut values = [0; LENGTH];
            let parts: Vec<&str> = value.trim().split(',').skip(6).collect();
            for (i, part) in parts.iter().enumerate().take(LENGTH) {
                if let Ok(number) = part.parse::<isize>() {
                    values[i] = number;
                }
            }
            burst.push(values);
        }
    }

    Ok(burst)
}

fn ave_std(data: &[f32]) -> (f32, f32) {
    let n = data.len();
    let sum: f32 = data.iter().sum();
    let average = sum / n as f32;

    let sum_squared_diff: f32 = data.iter().map(|x| (x - average).powi(2)).sum();
    let variance = sum_squared_diff / n as f32;
    let std_deviation = variance.sqrt();

    (average, std_deviation)
}
