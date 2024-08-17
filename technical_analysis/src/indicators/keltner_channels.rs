use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;
use crate::indicators::ExponentialMovingAverage;

pub struct KeltnerChannels {
    ema: ExponentialMovingAverage,
    atr_sum: IndicatorValue,
    atr_buffer: CircularBuffer,
    period_reciprocal: IndicatorValue,
    multiplier: IndicatorValue,
}

pub struct KeltnerChannelsOutput {
    pub upper_band: IndicatorValue,
    pub middle_band: IndicatorValue,
    pub lower_band: IndicatorValue,
}

impl KeltnerChannels {
    #[inline]
    pub fn new(period: usize, multiplier: f64) -> Self {
        let period_reciprocal = 1.0 / period as f64;
        KeltnerChannels {
            ema: ExponentialMovingAverage::new(period),
            atr_sum: 0.0.into(),
            atr_buffer: CircularBuffer::new(period),
            period_reciprocal: period_reciprocal.into(),
            multiplier: multiplier.into(),
        }
    }
}

impl Default for KeltnerChannels {
    fn default() -> Self {
        KeltnerChannels::new(20, 2.0)
    }
}

impl Indicator for KeltnerChannels {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = KeltnerChannelsOutput;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        let true_range = high - low;

        let old_value = if self.atr_buffer.is_full() {
            self.atr_buffer.push(true_range)
        } else {
            self.atr_buffer.push(true_range);
            0.0.into()
        };

        self.atr_sum += true_range - old_value;

        let middle_band = self.ema.next(close);
        let atr_value = self.atr_sum * self.period_reciprocal;
        let upper_band = middle_band + self.multiplier * atr_value;
        let lower_band = middle_band - self.multiplier * atr_value;

        KeltnerChannelsOutput {
            upper_band,
            middle_band,
            lower_band,
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(KeltnerChannelsOutput {
            upper_band: 0.0.into(),
            middle_band: 0.0.into(),
            lower_band: 0.0.into(),
        }, |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.ema.reset();
        self.atr_buffer.clear();
        self.atr_sum = 0.0.into();
    }
}
