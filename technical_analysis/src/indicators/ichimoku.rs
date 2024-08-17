use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct IchimokuClouds {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    tenkan_period: usize,
    kijun_period: usize,
    senkou_b_period: usize,
    current_max_high: IndicatorValue,
    current_min_low: IndicatorValue,
}

pub struct IchimokuCloudsOutput {
    pub tenkan_sen: IndicatorValue,
    pub kijun_sen: IndicatorValue,
    pub senkou_span_a: IndicatorValue,
    pub senkou_span_b: IndicatorValue,
    pub chikou_span: IndicatorValue,
}

impl IchimokuClouds {
    #[inline]
    pub fn new(tenkan_period: usize, kijun_period: usize, senkou_b_period: usize) -> Self {
        IchimokuClouds {
            high_buffer: CircularBuffer::new(senkou_b_period),
            low_buffer: CircularBuffer::new(senkou_b_period),
            tenkan_period,
            kijun_period,
            senkou_b_period,
            current_max_high: IndicatorValue::from(0.0),
            current_min_low: IndicatorValue::from(0.0),
        }
    }

    #[inline]
    fn update_max_min(&mut self, new_high: IndicatorValue, new_low: IndicatorValue) {
        if new_high >= self.current_max_high || self.high_buffer.get(0) == self.current_max_high {
            self.current_max_high = self.high_buffer.iter().max().unwrap();
        }
        if new_low <= self.current_min_low || self.low_buffer.get(0) == self.current_min_low {
            self.current_min_low = self.low_buffer.iter().min().unwrap();
        }
    }

    #[inline]
    fn calculate_tenkan_sen(&self) -> IndicatorValue {
        let tenkan_high = self.high_buffer.iter().take(self.tenkan_period).max().unwrap();
        let tenkan_low = self.low_buffer.iter().take(self.tenkan_period).min().unwrap();
        (tenkan_high + tenkan_low) / 2.0.into()
    }

    #[inline]
    fn calculate_kijun_sen(&self) -> IndicatorValue {
        let kijun_high = self.high_buffer.iter().take(self.kijun_period).max().unwrap();
        let kijun_low = self.low_buffer.iter().take(self.kijun_period).min().unwrap();
        (kijun_high + kijun_low) / 2.0.into()
    }

    #[inline]
    fn calculate_senkou_span_b(&self) -> IndicatorValue {
        let senkou_b_high = self.high_buffer.iter().take(self.senkou_b_period).max().unwrap();
        let senkou_b_low = self.low_buffer.iter().take(self.senkou_b_period).min().unwrap();
        (senkou_b_high + senkou_b_low) / 2.0.into()
    }
}

impl Default for IchimokuClouds {
    fn default() -> Self {
        IchimokuClouds::new(9, 26, 52) // Default parameters
    }
}

impl Indicator for IchimokuClouds {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IchimokuCloudsOutput;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        let old_high = self.high_buffer.push(high);
        let old_low = self.low_buffer.push(low);

        if old_high == self.current_max_high || old_low == self.current_min_low {
            self.update_max_min(high, low);
        } else {
            self.current_max_high = self.current_max_high.max(high);
            self.current_min_low = self.current_min_low.min(low);
        }

        let tenkan_sen = self.calculate_tenkan_sen();
        let kijun_sen = self.calculate_kijun_sen();
        let senkou_span_a = (tenkan_sen + kijun_sen) / 2.0.into();
        let senkou_span_b = self.calculate_senkou_span_b();

        let chikou_span = close;

        IchimokuCloudsOutput {
            tenkan_sen,
            kijun_sen,
            senkou_span_a,
            senkou_span_b,
            chikou_span,
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(
            IchimokuCloudsOutput {
                tenkan_sen: 0.0.into(),
                kijun_sen: 0.0.into(),
                senkou_span_a: 0.0.into(),
                senkou_span_b: 0.0.into(),
                chikou_span: 0.0.into(),
            },
            |_, &value| self.next(value),
        )
    }

    #[inline]
    fn reset(&mut self) {
        self.high_buffer.clear();
        self.low_buffer.clear();
        self.current_max_high = 0.0.into();
        self.current_min_low = 0.0.into();
    }
}
