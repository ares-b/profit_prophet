use crate::indicators::Indicator;
use crate::{IndicatorValue, CircularBuffer};

pub struct AverageTrueRange {
    true_ranges: CircularBuffer,
    prev_close: Option<IndicatorValue>,
    running_sum: IndicatorValue,
}

impl AverageTrueRange {
    #[inline]
    pub fn new(period: usize) -> Self {
        AverageTrueRange {
            true_ranges: CircularBuffer::new(period),
            prev_close: None,
            running_sum: 0.0.into(),
        }
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Indicator for AverageTrueRange {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;

        let true_range = match self.prev_close {
            Some(prev_close) => {
                let tr1 = high - low;
                let tr2 = (high - prev_close).abs();
                let tr3 = (low - prev_close).abs();
                tr1.max(tr2).max(tr3)
            }
            None => high - low,
        };

        if let Some(oldest_tr) = self.true_ranges.push(true_range) {
            self.running_sum += true_range - oldest_tr;
        } else {
            self.running_sum += true_range;
        }

        self.prev_close = Some(close);

        self.running_sum / (self.true_ranges.len() as f64).into()
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(IndicatorValue::from(0.0), |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.true_ranges.clear();
        self.prev_close = None;
        self.running_sum = 0.0.into();
    }
}