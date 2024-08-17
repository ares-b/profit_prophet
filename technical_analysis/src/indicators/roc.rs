use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct RateOfChange {
    buffer: CircularBuffer,
}

impl RateOfChange {
    #[inline]
    pub fn new(period: usize) -> Self {
        RateOfChange {
            buffer: CircularBuffer::new(period),
        }
    }
}

impl Default for RateOfChange {
    fn default() -> Self {
        Self::new(12)
    }
}

impl Indicator for RateOfChange {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = self.buffer.push(input);

        if old_value == 0.0.into() {
            0.0.into()
        } else {
            ((input - old_value) / old_value) * 100.0.into()
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut result = 0.0.into();
        for &value in input {
            result = self.next(value);
        }
        result
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
    }
}
