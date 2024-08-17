use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct SimpleMovingAverage {
    buffer: CircularBuffer,
    sum: IndicatorValue,
    period_reciprocal: IndicatorValue,
}

impl SimpleMovingAverage {
    #[inline]
    pub fn new(period: usize) -> Self {
        SimpleMovingAverage {
            buffer: CircularBuffer::new(period),
            sum: 0.0.into(),
            period_reciprocal: IndicatorValue::from(1.0) / IndicatorValue::from(period),
        }
    }
}

impl Default for SimpleMovingAverage {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Indicator for SimpleMovingAverage {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = self.buffer.push(input).unwrap_or(0.0.into());
        self.sum += input - old_value;
        self.sum * self.period_reciprocal
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut result = 0.0.into();
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
