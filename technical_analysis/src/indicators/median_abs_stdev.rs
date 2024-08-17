use crate::indicators::{SimpleMovingMedian, Indicator};
use crate::IndicatorValue;

#[derive(Debug, Clone)]
pub struct MedianAbsoluteStandardDeviation {
    median_calculator: SimpleMovingMedian,
}

impl MedianAbsoluteStandardDeviation {
    pub fn new(period: usize) -> Self {
        MedianAbsoluteStandardDeviation {
            median_calculator: SimpleMovingMedian::new(period),
        }
    }
}

impl Indicator for MedianAbsoluteStandardDeviation {
    type Output = IndicatorValue;
    type Input = IndicatorValue;

    fn next(&mut self, input: Self::Input) -> Self::Output {
        let current_median = self.median_calculator.next(input);
        
        input - current_median
    }

    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(IndicatorValue::from(0.0), |_, &value| self.next(value))
    }

    fn reset(&mut self) {
        self.median_calculator.reset();
    }
}
