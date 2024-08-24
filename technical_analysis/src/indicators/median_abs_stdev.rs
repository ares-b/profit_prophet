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
    type Output = Option<IndicatorValue>;
    type Input = IndicatorValue;

    fn next(&mut self, input: Self::Input) -> Self::Output {
        match self.median_calculator.next(input) {
            None => None,
            Some(current_median) => Some(input - current_median),
        }
    }

    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut result = None;
        for &value in input.iter() {
            result = self.next(value);
        }
        result
    }

    fn reset(&mut self) {
        self.median_calculator.reset();
    }
}
