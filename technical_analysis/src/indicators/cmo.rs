use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct ChandeMomentumOscillator {
    buffer: CircularBuffer,
}

impl ChandeMomentumOscillator {
    #[inline]
    pub fn new(period: usize) -> Self {
        ChandeMomentumOscillator {
            buffer: CircularBuffer::new(period),
        }
    }
}

impl Default for ChandeMomentumOscillator {
    fn default() -> Self {
        ChandeMomentumOscillator::new(14)
    }
}

impl Indicator for ChandeMomentumOscillator {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        self.buffer.push(input);

        let (mut sum_up, mut sum_down) = (IndicatorValue::from(0.0), IndicatorValue::from(0.0));

        for x in self.buffer.iter() {
            if x > 0.0.into() {
                sum_up += x;
            } else if x < 0.0.into() {
                sum_down -= x;
            }
        }

        if sum_up + sum_down == 0.0.into() {
            0.0.into()
        } else {
            IndicatorValue::from(100.0) * (sum_up - sum_down) / (sum_up + sum_down)
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
    }
}
