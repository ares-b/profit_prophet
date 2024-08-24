use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct DonchianChannels {
    pub high_buffer: CircularBuffer,
    pub low_buffer: CircularBuffer,
    pub current_max: IndicatorValue,
    pub current_min: IndicatorValue,
}

#[derive(Debug, PartialEq)]
pub struct DonchianChannelsOutput {
    pub upper_band: IndicatorValue,
    pub lower_band: IndicatorValue,
    pub middle_band: IndicatorValue,
}

impl DonchianChannels {
    #[inline]
    pub fn new(period: usize) -> Self {
        DonchianChannels {
            high_buffer: CircularBuffer::new(period),
            low_buffer: CircularBuffer::new(period),
            current_max: IndicatorValue::min(),
            current_min: IndicatorValue::max(),
        }
    }
}

impl Default for DonchianChannels {
    fn default() -> Self {
        DonchianChannels::new(20)
    }
}

impl Indicator for DonchianChannels {
    type Input = (IndicatorValue, IndicatorValue);
    type Output = Option<DonchianChannelsOutput>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low) = input;

        let old_high = self.high_buffer.push(high);
        let old_low = self.low_buffer.push(low);

        if old_high.is_some() && self.current_max == old_high.unwrap() {
            self.current_max = self.high_buffer.iter().max().copied().unwrap_or(IndicatorValue::min());
        } else {
            self.current_max = self.current_max.max(high);
        }

        if old_low.is_some() && self.current_min == old_low.unwrap() {
            self.current_min = self.low_buffer.iter().min().copied().unwrap_or(IndicatorValue::max());
        } else {
            self.current_min = self.current_min.min(low);
        }

        if !self.high_buffer.is_full() || !self.low_buffer.is_full() {
            return None;
        }

        let upper_band = self.current_max;
        let lower_band = self.current_min;
        let middle_band = (upper_band + lower_band) / 2.0.into();

        Some(DonchianChannelsOutput {
            upper_band,
            middle_band,
            lower_band,
        })
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(None, |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.high_buffer.clear();
        self.low_buffer.clear();
        self.current_max = IndicatorValue::min();
        self.current_min = IndicatorValue::max();
    }
}
