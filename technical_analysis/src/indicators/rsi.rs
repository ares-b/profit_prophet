use crate::indicators::{Indicator, ExponentialMovingAverage};
use crate::IndicatorValue;

pub struct RelativeStrengthIndex {
    gain_ema: ExponentialMovingAverage,
    loss_ema: ExponentialMovingAverage,
    prev_value: Option<IndicatorValue>,
    is_first: bool,
}

impl RelativeStrengthIndex {
    #[inline(always)]
    pub fn new(period: usize) -> Self {
        RelativeStrengthIndex {
            gain_ema: ExponentialMovingAverage::new(period),
            loss_ema: ExponentialMovingAverage::new(period),
            prev_value: None,
            is_first: true,
        }
    }
}

impl Default for RelativeStrengthIndex {
    fn default() -> Self {
        RelativeStrengthIndex::new(14)
    }
}

impl Indicator for RelativeStrengthIndex {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline(always)]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        // If this is the first value, just store it and return 50.0
        if self.is_first {
            self.prev_value = Some(input);
            self.is_first = false;
            return 50.0.into();
        }

        // Calculate the change
        let change = input - self.prev_value.unwrap();
        self.prev_value = Some(input);

        // Calculate the gain and loss
        let gain = if change > 0.0.into() { change } else { 0.0.into() };
        let loss = if change < 0.0.into() { -change } else { 0.0.into() };

        // Update the EMAs for gains and losses
        let avg_gain = self.gain_ema.next(gain);
        let avg_loss = self.loss_ema.next(loss);

        // Compute the Relative Strength (RS)
        let rs = if avg_loss == 0.0.into() {
            IndicatorValue::from(100.0)
        } else {
            avg_gain / avg_loss
        };

        let rsi = if avg_gain == 0.0.into() && avg_loss == 0.0.into() {
            IndicatorValue::from(50.0) // Return 50 for a constant series
        } else {
            IndicatorValue::from(100.0) - (IndicatorValue::from(100.0) / (IndicatorValue::from(1.0) + rs))
        };

        rsi
    }

    #[inline(always)]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(50.0.into(), |_, &value| self.next(value))
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.gain_ema.reset();
        self.loss_ema.reset();
        self.prev_value = None;
        self.is_first = true;
    }
}
