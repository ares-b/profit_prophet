pub use crate::indicators::Indicator;
use crate::CircularBuffer;

#[cfg(feature = "simd")]
use std::simd::{f64x4, prelude::SimdFloat};

pub struct StandardDeviation {
    buffer: CircularBuffer,
    sum: f64,
    sum_of_squares: f64,
}

impl StandardDeviation {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        StandardDeviation {
            buffer: CircularBuffer::new(period),
            sum: 0.0,
            sum_of_squares: 0.0,
        }
    }
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Indicator for StandardDeviation {
    type Output = f64;
    
    #[inline(always)]
    fn next(&mut self, input: f64) -> Self::Output {
        let old_value = self.buffer.push(input);
        self.sum -= old_value;
        self.sum_of_squares -= old_value * old_value;

        self.sum += input;
        self.sum_of_squares += input * input;

        let mean = self.sum / self.buffer.len() as f64;
        let variance = (self.sum_of_squares / self.buffer.len() as f64) - (mean * mean);

        variance.sqrt()
    }

    #[cfg(feature = "simd")]
    #[inline(always)]
    fn next_chunk(&mut self, input: &[f64]) -> Self::Output {
        let mut result = 0.0;

        for chunk in input.chunks_exact(4) {
            let values = f64x4::from_slice(chunk);
            let sum_vec = values.reduce_sum();
            let sum_of_squares_vec = (values * values).reduce_sum();

            for &value in chunk {
                let old_value = self.buffer.push(value);
                self.sum -= old_value;
                self.sum_of_squares -= old_value * old_value;
            }

            self.sum += sum_vec;
            self.sum_of_squares += sum_of_squares_vec;

            let mean = self.sum / self.buffer.len() as f64;
            let variance = (self.sum_of_squares / self.buffer.len() as f64) - (mean * mean);

            result = variance.sqrt();
        }

        result
    }

    #[cfg(not(feature = "simd"))]
    #[inline(always)]
    fn next_chunk(&mut self, input: &[f64]) -> Self::Output {
        input.iter().fold(0.0, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.sum = 0.0;
        self.sum_of_squares = 0.0;
        self.buffer.clear();
    }
}