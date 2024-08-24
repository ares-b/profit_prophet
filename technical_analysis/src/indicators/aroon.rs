use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct Aroon {
    high_buffer: CircularBuffer,
    low_buffer: CircularBuffer,
    period: usize,
    period_reciprocal: IndicatorValue,
    current_highest_index: usize,
    current_lowest_index: usize,
    current_highest_value: IndicatorValue,
    current_lowest_value: IndicatorValue,
}

#[derive(Debug, PartialEq)]
pub struct AroonOutput {
    pub aroon_up: IndicatorValue,
    pub aroon_down: IndicatorValue,
}

impl Aroon {
    #[inline]
    pub fn new(period: usize) -> Self {
        Aroon {
            high_buffer: CircularBuffer::new(period),
            low_buffer: CircularBuffer::new(period),
            period,
            period_reciprocal: IndicatorValue::from(1.0) / IndicatorValue::from(period),
            current_highest_index: 0,
            current_lowest_index: 0,
            current_highest_value: IndicatorValue::from(0.0),
            current_lowest_value: IndicatorValue::from(IndicatorValue::max()),
        }
    }

    #[inline]
    fn update_highest(&mut self, value: IndicatorValue) {
        self.high_buffer.push(value);

        if value >= self.current_highest_value {
            self.current_highest_value = value;
            self.current_highest_index = 0;
        } else {
            self.current_highest_index += 1;
            if self.current_highest_index >= self.period {
                self.recalculate_highest();
            }
        }
    }

    #[inline]
    fn update_lowest(&mut self, value: IndicatorValue) {
        self.low_buffer.push(value);

        if value <= self.current_lowest_value {
            // New lowest value, reset the index
            self.current_lowest_value = value;
            self.current_lowest_index = 0;
        } else {
            // Increment the index and check if we need to find a new lowest
            self.current_lowest_index += 1;
            if self.current_lowest_index >= self.period {
                self.recalculate_lowest();
            }
        }
    }

    #[inline]
    fn recalculate_highest(&mut self) {
        let (index, max_value) = self.high_buffer
            .iter()
            .copied()
            .enumerate()
            .max_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        self.current_highest_index = index;
        self.current_highest_value = max_value;
    }

    #[inline]
    fn recalculate_lowest(&mut self) {
        let (index, min_value) = self.low_buffer
            .iter()
            .copied()
            .enumerate()
            .min_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        self.current_lowest_index = index;
        self.current_lowest_value = min_value;
    }
}

impl Default for Aroon {
    fn default() -> Self {
        Aroon::new(14)
    }
}

impl Indicator for Aroon {
    type Input = (IndicatorValue, IndicatorValue);
    type Output = Option<AroonOutput>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low) = input;
        self.update_highest(high);
        self.update_lowest(low);

        if !self.high_buffer.is_full() {
            return None;
        }

        Some(AroonOutput {
            aroon_up: IndicatorValue::from(self.period - self.current_highest_index) * self.period_reciprocal * IndicatorValue::from(100.0),
            aroon_down: IndicatorValue::from(self.period - self.current_lowest_index)  * self.period_reciprocal * IndicatorValue::from(100.0),
        })
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
        self.high_buffer.clear();
        self.low_buffer.clear();
        self.current_highest_index = 0;
        self.current_lowest_index = 0;
        self.current_highest_value = 0.0.into();
        self.current_lowest_value = IndicatorValue::max();
    }
}
