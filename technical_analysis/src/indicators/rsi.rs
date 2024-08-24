use crate::indicators::Indicator;
use crate::IndicatorValue;

pub struct RelativeStrengthIndex {
    prev_value: Option<IndicatorValue>,
    avg_gain: IndicatorValue,
    avg_loss: IndicatorValue,
    count: usize,
    period: usize,
    period_reciprocal: IndicatorValue,
}

impl RelativeStrengthIndex {
    #[inline]
    pub fn new(period: usize) -> Self {
        Self {
            prev_value: None,
            avg_gain: IndicatorValue::from(0.0),
            avg_loss: IndicatorValue::from(0.0),
            count: 0,
            period,
            period_reciprocal: IndicatorValue::from(1.0) / IndicatorValue::from(period),
        }
    }

    #[inline]
    fn calculate_rsi(&self) -> IndicatorValue {
        let rs = self.avg_gain / self.avg_loss;
        IndicatorValue::from(100.0) - (IndicatorValue::from(100.0) / (IndicatorValue::from(1.0) + rs))
    }
}

impl Default for RelativeStrengthIndex {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Indicator for RelativeStrengthIndex {
    type Input = IndicatorValue;
    type Output = Option<IndicatorValue>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        if let Some(previous_value) = self.prev_value {
            let change = input - previous_value;

            let gain = if change > IndicatorValue::from(0.0) { change } else { IndicatorValue::from(0.0) };
            let loss = if change < IndicatorValue::from(0.0) { -change } else { IndicatorValue::from(0.0) };

            self.count += 1;

            if self.count < self.period {
                // Accumulate initial sums for the first period
                self.avg_gain += gain;
                self.avg_loss += loss;
                self.prev_value = Some(input);
                return None;
            } else if self.count == self.period {
                // Finalize the initial average gain and loss
                self.avg_gain += gain;
                self.avg_loss += loss;
                self.avg_gain *= self.period_reciprocal;
                self.avg_loss *= self.period_reciprocal;
            } else {
                // Update the average gains and losses using the smoothing factor
                self.avg_gain = (self.avg_gain * IndicatorValue::from(self.period - 1) + gain) * self.period_reciprocal;
                self.avg_loss = (self.avg_loss * IndicatorValue::from(self.period - 1) + loss) * self.period_reciprocal;
            }

            self.prev_value = Some(input);

            // Handle special cases and return RSI
            if self.avg_gain == IndicatorValue::from(0.0) && self.avg_loss == IndicatorValue::from(0.0) {
                Some(IndicatorValue::from(50.0))  // Case: Constant prices
            } else if self.avg_loss == IndicatorValue::from(0.0) {
                Some(IndicatorValue::from(100.0)) // Case: Increasing prices only, no losses
            } else if self.avg_gain == IndicatorValue::from(0.0) {
                Some(IndicatorValue::from(0.0))   // Case: Decreasing prices only, no gains
            } else {
                Some(self.calculate_rsi())        // Standard RSI calculation
            }
        } else {
            self.prev_value = Some(input);
            None
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
        self.prev_value = None;
        self.avg_gain = IndicatorValue::from(0.0);
        self.avg_loss = IndicatorValue::from(0.0);
        self.count = 0;
    }
}
