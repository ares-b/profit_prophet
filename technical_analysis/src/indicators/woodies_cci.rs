use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct WoodiesCCI {
    buffer: CircularBuffer,
    period: usize,
    running_sum: IndicatorValue,
    running_sum_deviation: IndicatorValue,
}

impl WoodiesCCI {
    #[inline]
    pub fn new(period: usize) -> Self {
        WoodiesCCI {
            buffer: CircularBuffer::new(period),
            period,
            running_sum: 0.0.into(),
            running_sum_deviation: 0.0.into(),
        }
    }

    #[inline]
    fn update_deviation(&mut self, new_value: IndicatorValue, old_value: IndicatorValue, mean: IndicatorValue) {
        let old_deviation = (old_value - mean).abs();
        let new_deviation = (new_value - mean).abs();
        self.running_sum_deviation += new_deviation - old_deviation;
    }
}

impl Default for WoodiesCCI {
    fn default() -> Self {
        WoodiesCCI::new(20)
    }
}

impl Indicator for WoodiesCCI {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        let typical_price = (high + low + close) / 3.0.into();

        let oldest_value = if self.buffer.is_full() {
            Some(self.buffer.push(typical_price))
        } else {
            self.buffer.push(typical_price);
            None
        };

        self.running_sum += typical_price - oldest_value.unwrap_or(0.0.into());
        let mean = self.running_sum / self.period.into();

        if let Some(old_value) = oldest_value {
            self.update_deviation(typical_price, old_value, mean);
        } else {
            self.running_sum_deviation = (0..self.buffer.len())
                .map(|i| (self.buffer.get(i) - mean).abs())
                .sum();
        }

        let mean_deviation = self.running_sum_deviation / self.period.into();

        (typical_price - mean) / (mean_deviation * 0.015.into())
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
        self.running_sum = 0.0.into();
        self.running_sum_deviation = 0.0.into();
    }
}
