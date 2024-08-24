use crate::indicators::Indicator;
use crate::IndicatorValue;

pub struct ExponentialMovingAverage {
    multiplier: IndicatorValue,
    current_ema: Option<IndicatorValue>,
    period: usize,
    count: usize,
}

impl ExponentialMovingAverage {
    #[inline]
    pub fn new(period: usize) -> Self {
        let multiplier = IndicatorValue::from(2.0) / IndicatorValue::from(period + 1);
        ExponentialMovingAverage {
            multiplier,
            current_ema: None,
            period,
            count: 0
        }
    }

    #[inline]
    pub fn set_ema(&mut self, ema: IndicatorValue) {
        self.current_ema = Some(ema);
        self.count = self.period;
    }
}

impl Default for ExponentialMovingAverage {
    fn default() -> Self {
        ExponentialMovingAverage::new(14)
    }
}

impl Indicator for ExponentialMovingAverage {
    type Input = IndicatorValue;
    type Output = Option<IndicatorValue>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let ema = match self.current_ema {
            Some(previous_ema) => {
                (self.multiplier * input) + ((IndicatorValue::from(1.0) - self.multiplier) * previous_ema)
            }
            None => input,
        };
        self.current_ema = Some(ema);
        self.count += 1;

        if self.count < self.period {
            return None;
        }
        Some(ema)
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
        self.current_ema = None;
        self.count = 0;
    }
}
