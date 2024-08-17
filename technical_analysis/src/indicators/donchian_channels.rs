use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct DonchianChannels {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    current_max: IndicatorValue,
    current_min: IndicatorValue,
    max_index: usize,
    min_index: usize,
}

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
            current_max: IndicatorValue::from(0.0),
            current_min: IndicatorValue::from(0.0),
            max_index: 0,
            min_index: 0,
        }
    }

    #[inline]
    fn update_extremes(&mut self, high: IndicatorValue, low: IndicatorValue) {
        // Update current max
        if high > self.current_max || self.high_buffer.len() == 0 {
            self.current_max = high;
            self.max_index = self.high_buffer.len() - 1;
        } else if self.high_buffer.get(0) == self.current_max {
            self.current_max = self.high_buffer.iter().max().unwrap();
            self.max_index = self.high_buffer.iter().position(|x| x == self.current_max).unwrap();
        }

        if low < self.current_min || self.low_buffer.len() == 0 {
            self.current_min = low;
            self.min_index = self.low_buffer.len() - 1;
        } else if self.low_buffer.get(0) == self.current_min {
            self.current_min = self.low_buffer.iter().min().unwrap();
            self.min_index = self.low_buffer.iter().position(|x| x == self.current_min).unwrap();
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
    type Output = DonchianChannelsOutput;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low) = input;

        let old_high = if self.high_buffer.is_full() {
            Some(self.high_buffer.push(high))
        } else {
            self.high_buffer.push(high);
            None
        };

        let old_low = if self.low_buffer.is_full() {
            Some(self.low_buffer.push(low))
        } else {
            self.low_buffer.push(low);
            None
        };

        if let Some(old_high) = old_high {
            if old_high == self.current_max {
                self.update_extremes(high, low);
            }
        } else {
            self.current_max = high.max(self.current_max);
        }

        if let Some(old_low) = old_low {
            if old_low == self.current_min {
                self.update_extremes(high, low);
            }
        } else {
            self.current_min = low.min(self.current_min);
        }

        // Calculate bands
        let upper_band = self.current_max;
        let lower_band = self.current_min;
        let middle_band = (upper_band + lower_band) / 2.0.into();

        DonchianChannelsOutput {
            upper_band,
            middle_band,
            lower_band,
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(
            DonchianChannelsOutput {
                upper_band: self.current_max,
                middle_band: self.current_max,
                lower_band: self.current_min,
            },
            |_, &value| self.next(value),
        )
    }

    #[inline]
    fn reset(&mut self) {
        self.high_buffer.clear();
        self.low_buffer.clear();
        self.current_max = 0.0.into();
        self.current_min = 0.0.into();
        self.max_index = 0;
        self.min_index = 0;
    }
}
