use crate::indicators::Indicator;
use crate::{IndicatorValue, CircularBuffer};

pub struct AverageTrueRange {
    true_ranges: CircularBuffer,
    prev_close: Option<IndicatorValue>,
    running_sum: IndicatorValue,
    period_reciprocal: IndicatorValue,
}

impl AverageTrueRange {
    #[inline]
    pub fn new(period: usize) -> Self {
        AverageTrueRange {
            true_ranges: CircularBuffer::new(period),
            prev_close: None,
            running_sum: 0.0.into(),
            period_reciprocal: IndicatorValue::from(1.0) / IndicatorValue::from(period),
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
    type Output = Option<IndicatorValue>;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;

        let true_range = if let Some(prev_close) = self.prev_close {
            let tr1 = high - low;
            let tr2 = (high - prev_close).abs();
            let tr3 = (low - prev_close).abs();
            tr1.max(tr2).max(tr3)
        } else {
            high - low
        };

        self.prev_close = Some(close);

        if let Some(oldest_tr) = self.true_ranges.push(true_range) {
            self.running_sum += true_range - oldest_tr;
        } else {
            self.running_sum += true_range;
        }

        if !self.true_ranges.is_full() {
            None
        } else {
            Some(self.running_sum * self.period_reciprocal)
        }
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(None, |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.true_ranges.clear();
        self.prev_close = None;
        self.running_sum = 0.0.into();
    }
}
