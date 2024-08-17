use crate::indicators::Indicator;
use crate::IndicatorValue;
use crate::indicators::ExponentialMovingAverage;

pub struct MovingAverageConvergenceDivergence {
    short_ema: ExponentialMovingAverage,
    long_ema: ExponentialMovingAverage,
    signal_ema: ExponentialMovingAverage,
}

pub struct MovingAverageConvergenceDivergenceOutput {
    pub macd_value: IndicatorValue,
    pub signal_value: IndicatorValue,
    pub histogram_value: IndicatorValue,
}

impl MovingAverageConvergenceDivergence {
    #[inline]
    pub fn new(short_period: usize, long_period: usize, signal_period: usize) -> Self {
        MovingAverageConvergenceDivergence {
            short_ema: ExponentialMovingAverage::new(short_period),
            long_ema: ExponentialMovingAverage::new(long_period),
            signal_ema: ExponentialMovingAverage::new(signal_period),
        }
    }
}

impl Default for MovingAverageConvergenceDivergence {
    fn default() -> Self {
        MovingAverageConvergenceDivergence::new(12, 26, 9)
    }
}

impl Indicator for MovingAverageConvergenceDivergence {
    type Input = IndicatorValue;
    type Output = MovingAverageConvergenceDivergenceOutput;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let short_ema_value = self.short_ema.next(input);
        let long_ema_value = self.long_ema.next(input);
        let macd_value = short_ema_value - long_ema_value;
        let signal_value = self.signal_ema.next(macd_value);
        let histogram_value = macd_value - signal_value;

        MovingAverageConvergenceDivergenceOutput {
            macd_value,
            signal_value,
            histogram_value,
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(MovingAverageConvergenceDivergenceOutput {
            macd_value: 0.0.into(),
            signal_value: 0.0.into(),
            histogram_value: 0.0.into(),
        }, |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.short_ema.reset();
        self.long_ema.reset();
        self.signal_ema.reset();
    }
}
