use crate::{CircularBuffer, IndicatorValue};
use crate::indicators::Indicator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HighOutput {
    pub high_value: IndicatorValue,
    pub high_index: usize,
}

#[derive(Debug)]
pub struct High {
    buffer: CircularBuffer,
    current_high_value: Option<IndicatorValue>,
    current_high_index: Option<usize>,
}

impl High {
    pub fn new(period: usize) -> Self {
        High {
            buffer: CircularBuffer::new(period),
            current_high_value: None,
            current_high_index: None,
        }
    }

    #[inline]
    fn update_high(&mut self, input: IndicatorValue, old_value: Option<IndicatorValue>, input_index: usize) {
        if old_value.is_some() && self.current_high_value == old_value {
            self.current_high_value = None;
            self.current_high_index = None;
            for (i, &value) in self.buffer.iter().enumerate() {
                if self.current_high_value.map_or(true, |high| value > high) {
                    self.current_high_value = Some(value);
                    self.current_high_index = Some(i);
                }
            }
        } else if self.current_high_value.map_or(true, |high| input > high) {
            self.current_high_value = Some(input);
            self.current_high_index = Some(input_index);
        }
    }
    
}

impl Indicator for High {
    type Input = IndicatorValue;
    type Output = Option<HighOutput>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let next_index = self.buffer.index();
        let old_value = self.buffer.push(input);

        self.update_high(input, old_value, next_index);

        if self.buffer.is_full() {
            return Some(HighOutput{
                high_value: self.current_high_value.unwrap(),
                high_index: self.current_high_index.unwrap(),
            })
        }
        None
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
        self.buffer.clear();
        self.current_high_value = None;
        self.current_high_index = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LowOutput {
    pub low_value: IndicatorValue,
    pub low_index: usize,
}

#[derive(Debug)]
pub struct Low {
    buffer: CircularBuffer,
    current_low_value: Option<IndicatorValue>,
    current_low_index: Option<usize>,
}

impl Low {
    pub fn new(period: usize) -> Self {
        Low {
            buffer: CircularBuffer::new(period),
            current_low_value: None,
            current_low_index: None,
        }
    }

    #[inline]
    fn update_low(&mut self, input: IndicatorValue, old_value: Option<IndicatorValue>, input_index: usize) {
        if old_value.is_some() && self.current_low_value == old_value {
            self.current_low_value = None;
            self.current_low_index = None;
            for (i, &value) in self.buffer.iter().enumerate() {
                if self.current_low_value.map_or(true, |low| value < low) {
                    self.current_low_value = Some(value);
                    self.current_low_index = Some(i);
                }
            }
        } else if self.current_low_value.map_or(true, |low| input < low) {
            self.current_low_value = Some(input);
            self.current_low_index = Some(input_index);
        }
    }


}

impl Indicator for Low {
    type Input = IndicatorValue;
    type Output = Option<LowOutput>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let next_index = self.buffer.index();
        let old_value = self.buffer.push(input);
        self.update_low(input, old_value, next_index);

        if self.buffer.is_full() {
            return Some(LowOutput{
                low_value: self.current_low_value.unwrap(),
                low_index: self.current_low_index.unwrap()
            })
        }
        None
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
        self.buffer.clear();
        self.current_low_value = None;
        self.current_low_index = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HighLowOutput {
    pub high_value: IndicatorValue,
    pub high_index: usize,
    pub low_value: IndicatorValue,
    pub low_index: usize,
}

#[derive(Debug)]
pub struct HighLow {
    buffer: CircularBuffer,
    current_high_value: Option<IndicatorValue>,
    current_high_index: Option<usize>,
    current_low_value: Option<IndicatorValue>,
    current_low_index: Option<usize>,
}

impl HighLow {
    pub fn new(period: usize) -> Self {
        HighLow {
            buffer: CircularBuffer::new(period),
            current_high_value: None,
            current_high_index: None,
            current_low_value: None,
            current_low_index: None,
        }
    }

    #[inline]
    fn update_high(&mut self, input: IndicatorValue, old_value: Option<IndicatorValue>, input_index: usize) {
        if old_value.is_some() && self.current_high_value == old_value {
            self.current_high_value = None;
            self.current_high_index = None;
            for (i, &value) in self.buffer.iter().enumerate() {
                if self.current_high_value.map_or(true, |high| value > high) {
                    self.current_high_value = Some(value);
                    self.current_high_index = Some(i);
                }
            }
        } else if self.current_high_value.map_or(true, |high| input > high) {
            self.current_high_value = Some(input);
            self.current_high_index = Some(input_index);
        }
    }

    #[inline]
    fn update_low(&mut self, input: IndicatorValue, old_value: Option<IndicatorValue>, input_index: usize) {
        if old_value.is_some() && self.current_low_value == old_value {
            self.current_low_value = None;
            self.current_low_index = None;
            for (i, &value) in self.buffer.iter().enumerate() {
                if self.current_low_value.map_or(true, |low| value < low) {
                    self.current_low_value = Some(value);
                    self.current_low_index = Some(i);
                }
            }
        } else if self.current_low_value.map_or(true, |low| input < low) {
            self.current_low_value = Some(input);
            self.current_low_index = Some(input_index);
        }
    }

    #[inline]
    pub fn buffer_is_full(&self) -> bool {
        self.buffer.is_full()
    }

}

impl Indicator for HighLow {
    type Input = IndicatorValue;
    type Output = Option<HighLowOutput>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let next_index = self.buffer.index();
        let old_value = self.buffer.push(input);
        self.update_high(input, old_value, next_index);
        self.update_low(input, old_value, next_index);

        if self.buffer.is_full() {
            return Some(HighLowOutput{
                high_value: self.current_high_value.unwrap(),
                high_index: self.current_high_index.unwrap(),
                low_value: self.current_low_value.unwrap(),
                low_index: self.current_low_index.unwrap()
            })
        }
        None
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
        self.buffer.clear();
        self.current_high_value = None;
        self.current_high_index = None;
        self.current_low_value = None;
        self.current_low_index = None;
    }
}
