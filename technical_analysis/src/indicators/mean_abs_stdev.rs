use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct MeanAbsDev {
    buffer: CircularBuffer,
    sum: IndicatorValue,
    reciprocal_period: IndicatorValue,
}

impl MeanAbsDev {
    #[inline]
    pub fn new(period: usize) -> Self {
        let reciprocal_period = 1.0 / period as f64;
        MeanAbsDev {
            buffer: CircularBuffer::new(period),
            sum: 0.0.into(),
            reciprocal_period: reciprocal_period.into(),
        }
    }
}

impl Default for MeanAbsDev {
    fn default() -> Self {
        MeanAbsDev::new(20)
    }
}

impl Indicator for MeanAbsDev {
    type Input = IndicatorValue;
    type Output = Option<IndicatorValue>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = self.buffer.push(input).unwrap_or(0.0.into());
        self.sum += input - old_value;

        if self.buffer.is_full() {
            let mean = self.sum * self.reciprocal_period;
            Some(
                self.buffer.iter().map(|&value| (value - mean).abs()).sum::<IndicatorValue>() * self.reciprocal_period
            )
        } else {
            None
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut result = None;
        for &value in input.iter() {
            result = self.next(value);
        }
        result
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
        self.sum = 0.0.into();
    }
}
