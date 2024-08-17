use crate::indicators::{Indicator, ExponentialMovingAverage};
use crate::IndicatorValue;

pub struct PercentagePriceOscillator {
    short_ema: ExponentialMovingAverage,
    long_ema: ExponentialMovingAverage,
    signal_ema: ExponentialMovingAverage,
}

pub struct PPOOutput {
    pub ppo_value: IndicatorValue,
    pub signal_value: IndicatorValue,
    pub histogram_value: IndicatorValue,
}

impl PercentagePriceOscillator {
    #[inline]
    pub fn new(short_period: usize, long_period: usize, signal_period: usize) -> Self {
        PercentagePriceOscillator {
            short_ema: ExponentialMovingAverage::new(short_period),
            long_ema: ExponentialMovingAverage::new(long_period),
            signal_ema: ExponentialMovingAverage::new(signal_period),
        }
    }
}

impl Default for PercentagePriceOscillator {
    fn default() -> Self {
        PercentagePriceOscillator::new(12, 26, 9)
    }
}

impl Indicator for PercentagePriceOscillator {
    type Input = IndicatorValue;
    type Output = PPOOutput;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let short_ema_value = self.short_ema.next(input);
        let long_ema_value = self.long_ema.next(input);
        
        let ppo_value = ((short_ema_value - long_ema_value) / long_ema_value) * 100.0.into();
        
        let signal_value = self.signal_ema.next(ppo_value);
        
        let histogram_value = ppo_value - signal_value;

        PPOOutput {
            ppo_value,
            signal_value,
            histogram_value,
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(PPOOutput {
            ppo_value: 0.0.into(),
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
