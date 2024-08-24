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
    type Output = Option<IndicatorValue>;
    type Input = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        match self.buffer.push(input) {
            Some(old_value) => {
                let roc = if old_value == IndicatorValue::from(0.0) {
                    IndicatorValue::from(0.0)
                } else {
                    ((input - old_value) / old_value) * IndicatorValue::from(100.0)
                };
                return Some(roc)
            },
            None => return None
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
    }
}
