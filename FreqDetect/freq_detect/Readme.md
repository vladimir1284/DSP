# Frequency Computation Library

This library provides two different methods for computing frequency from digital sampled data: FFT and Zero crossings. The implementations are written in Rust.

## Dependencies

- `realfft = "3.3.0"`
- `rustfft = "6.1.0"`

## Installation

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
frequency_computation = { git = "https://github.com/vladimir1284/DSP.git" }
```

## Usage

### FFT

The FFT implementation can be used as follows:

```rust
use freq_detect::fft::FFTProcessor;
const SAMPLING_RATE: usize = 100 * 1000 * 1000; // Sampling frequency
const LENGTH: usize = 511;

let signal: Vec<f64> = vec![0.0; LENGTH];
let mut fft_processor = FFTProcessor::new(LENGTH);

let frequency: f64 = fft_processor.freq_from_fft(&signal, SAMPLING_RATE);

println!("Frequency: {} Hz", frequency);
```

### Zero Crossings

The Zero crossings implementation can be used as follows:

```rust
use freq_detect::zero_crossings::ZeroCrossProcessor;
const SAMPLING_RATE: usize = 100 * 1000 * 1000; // Sampling frequency
const LENGTH: usize = 511;

let signal: Vec<f64> = vec![0.0; LENGTH];
let processor = ZeroCrossProcessor::new(LENGTH);

let frequency: f64 = processor.freq_from_crossings(&signal, SAMPLING_RATE);

println!("Frequency: {} Hz", frequency);
```

## Testing

Tests are written using 1000 time-series with 511 data points each. To run the tests, use the following command:

```shell
cargo test
```

### Performance
The Zero Crossings algorithms is 10 times faster than the FFT algorithm for the 
general case, like the 511 sample points of this example. However, for sample 
size power of 2, it is only 3 times faster. FFT gives better results reducing 
the standard deviation of the frequencies computed in the test 50 times.


## References

- `realfft`: https://crates.io/crates/realfft
- `rustfft`: https://crates.io/crates/rustfft

## Implementation Files

- FFT implementation: `src/fft.rs`
- Zero crossings implementation: `src/zero_crossings.rs`