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

#[derive(Debug)]
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
        
        self.current_highest_index += 1;

        if value >= self.current_highest_value {
            self.current_highest_value = value;
            self.current_highest_index = 0;
        } else if self.current_highest_index == self.period  {
            let (index, value) = self.high_buffer
                .iter()
                .copied()
                .enumerate()
                .fold((0, value), |a, b| if b.1 > a.1 { b } else { a });

            self.current_highest_index = index;
            self.current_highest_value = value;
        }
    }

    #[inline]
    fn update_lowest(&mut self, value: IndicatorValue) {
        self.low_buffer.push(value);
        self.current_lowest_index += 1;

        if value <= self.current_lowest_value {
            self.current_lowest_value = value;
            self.current_lowest_index = 0;
        } else if self.current_lowest_index == self.period {
            let (index, value) = self.low_buffer
                .iter()
                .copied()
                .enumerate()
                .fold((0, value), |a, b| if b.1 < a.1 { b } else { a });

            self.current_lowest_index = index;
            self.current_lowest_value = value;
        }
    }

    #[inline]
    fn calculate_aroon(&self) -> AroonOutput {
        let aroon_up: IndicatorValue   = IndicatorValue::from(self.period - self.current_highest_index) * self.period_reciprocal * IndicatorValue::from(100.0);
        let aroon_down: IndicatorValue = IndicatorValue::from(self.period - self.current_lowest_index)  * self.period_reciprocal * IndicatorValue::from(100.0);

        AroonOutput {
            aroon_up,
            aroon_down,
        }
    }
}

impl Default for Aroon {
    fn default() -> Self {
        Aroon::new(14)
    }
}

impl Indicator for Aroon {
    type Input = (IndicatorValue, IndicatorValue);
    type Output = AroonOutput;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low) = input;
        self.update_highest(high);
        self.update_lowest(low);
        self.calculate_aroon()
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(AroonOutput {
            aroon_up: 0.0.into(),
            aroon_down: 0.0.into(),
        }, |_, &value| self.next(value))
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
