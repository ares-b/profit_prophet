use crate::indicators::Indicator;
use crate::IndicatorValue;

pub struct ExponentialMovingAverage {
    multiplier: IndicatorValue,
    current_ema: Option<IndicatorValue>,
}

impl ExponentialMovingAverage {
    #[inline]
    pub fn new(period: usize) -> Self {
        let multiplier = IndicatorValue::from(2.0) / IndicatorValue::from(period + 1);
        ExponentialMovingAverage {
            multiplier,
            current_ema: None,
        }
    }
}

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        ExponentialMovingAverage::new(14)
    }
}

impl Indicator for ExponentialMovingAverage {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let ema = match self.current_ema {
            Some(previous_ema) => {
                (self.multiplier * input) + ((IndicatorValue::from(1.0) - self.multiplier) * previous_ema)
            }
            None => input, // Initialize EMA with the first input value
        };
        self.current_ema = Some(ema);
        ema
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(self.current_ema.unwrap_or(0.0.into()), |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.current_ema = None;
    }
}
