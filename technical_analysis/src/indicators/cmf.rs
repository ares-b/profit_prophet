use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct ChaikinMoneyFlow {
    buffer: CircularBuffer,
    period: usize,
    running_sum: IndicatorValue,
}

impl ChaikinMoneyFlow {
    #[inline]
    pub fn new(period: usize) -> Self {
        ChaikinMoneyFlow {
            buffer: CircularBuffer::new(period),
            period,
            running_sum: 0.0.into(),
        }
    }
}

impl Default for ChaikinMoneyFlow {
    fn default() -> Self {
        ChaikinMoneyFlow::new(20)
    }
}

impl Indicator for ChaikinMoneyFlow {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close, volume) = input;

        if high == low {
            return 0.0.into();
        }

        let mfv = ((close - low) - (high - close)) / (high - low) * volume;

        if self.buffer.is_full() {
            let oldest_value = self.buffer.push(mfv);
            self.running_sum += mfv - oldest_value;
        } else {
            self.buffer.push(mfv);
            self.running_sum += mfv;
        }

        self.running_sum / self.period.into()
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
        self.running_sum = 0.0.into();
    }
}
