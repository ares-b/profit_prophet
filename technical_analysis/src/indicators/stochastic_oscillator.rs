use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct StochasticOscillator {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    last_k: IndicatorValue,
}

pub struct StochasticOutput {
    pub k: IndicatorValue,
    pub d: IndicatorValue,
}

impl StochasticOscillator {
    #[inline]
    pub fn new(period: usize) -> Self {
        StochasticOscillator {
            high_buffer: CircularBuffer::new(period),
            low_buffer: CircularBuffer::new(period),
            last_k: 50.0.into(),
        }
    }
}

impl Default for StochasticOscillator {
    fn default() -> Self {
        StochasticOscillator::new(14)
    }
}

impl Indicator for StochasticOscillator {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = StochasticOutput;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        self.high_buffer.push(high);
        self.low_buffer.push(low);

        let highest_high = self.high_buffer.iter().max().unwrap();
        let lowest_low = self.low_buffer.iter().min().unwrap();

        let range = highest_high - lowest_low;
        let k = if range > 0.0.into() {
            (close - lowest_low) / range * 100.0.into()
        } else {
            self.last_k 
        };

        
        let d = (self.last_k + k) / 2.0.into();

        self.last_k = k;

        StochasticOutput { k, d }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(StochasticOutput {
            k: self.last_k,
            d: self.last_k,
        }, |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.high_buffer.clear();
        self.low_buffer.clear();
        self.last_k = 50.0.into();
    }
}
