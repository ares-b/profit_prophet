use crate::{CircularBuffer, IndicatorValue};
use crate::indicators::Indicator;

pub struct StandardDeviation {
    pub buffer: CircularBuffer,
    sum: IndicatorValue,
    sum_of_squares: IndicatorValue,
    period_reciprocal: IndicatorValue,
    bessel_correction_reciprocal: IndicatorValue,
    mean: IndicatorValue,
}

impl StandardDeviation {
    #[inline]
    pub fn new(period: usize) -> Self {
        let period_f64 = period as f64;
        StandardDeviation {
            buffer: CircularBuffer::new(period),
            sum: 0.0.into(),
            sum_of_squares: 0.0.into(),
            period_reciprocal: (1.0 / period_f64).into(),
            bessel_correction_reciprocal: (1.0 / (period_f64 - 1.0)).into(),
            mean: 0.0.into(),
        }
    }
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Indicator for StandardDeviation {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = self.buffer.push(input).unwrap_or(0.0.into());
        self.sum += input - old_value;
        let old_mean = self.mean;
        self.mean = self.sum * self.period_reciprocal;
        self.sum_of_squares += (input - self.mean) * (input - old_mean)
            - (old_value - old_mean) * (old_value - self.mean);

        (self.sum_of_squares * self.bessel_correction_reciprocal).sqrt()
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        for &value in input {
            self.next(value);
        }
        (self.sum_of_squares * self.bessel_correction_reciprocal).sqrt()
    }

    #[inline]
    fn reset(&mut self) {
        self.sum = 0.0.into();
        self.sum_of_squares = 0.0.into();
        self.mean = 0.0.into();
        self.buffer.clear();
    }
}
