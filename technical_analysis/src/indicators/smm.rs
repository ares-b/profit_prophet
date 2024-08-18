use crate::CircularBuffer;
use crate::indicators::Indicator;
use crate::IndicatorValue;

#[derive(Debug, Clone)]
pub struct SimpleMovingMedian {
    buffer: CircularBuffer,
    slice: Vec<IndicatorValue>,
    is_even: bool,
    median_index: usize,
    median_index_1: usize,
}

impl SimpleMovingMedian {
    #[inline]
    pub fn new(period: usize) -> Self {
        let median_index = period / 2;
        let is_even = period % 2 == 0;

        SimpleMovingMedian {
            buffer: CircularBuffer::new(period),
            slice: Vec::with_capacity(period),
            is_even,
            median_index,
            median_index_1: median_index.saturating_sub(is_even as usize),
        }
    }

}

impl Indicator for SimpleMovingMedian {
    type Output = Option<IndicatorValue>;
    type Input = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        
        let old_value = self.buffer.push(input);

        // Remove the old value from the sorted slice if the buffer was full
        if let Some(old_value) = old_value {
            if let Ok(pos) = self.slice.binary_search(&old_value) {
                self.slice.remove(pos);
            }
        }

        // Insert the new value into the sorted slice
        let pos = self.slice.binary_search(&input).unwrap_or_else(|e| e);
        self.slice.insert(pos, input);

        // If the buffer is not yet full, we return None (early return)
        if !self.buffer.is_full() {
            return None;
        }

        // Calculate and return the median
        if self.is_even {
            // For an even number of elements, return the average of the two middle elements
            Some(
                (self.slice[self.median_index_1] + self.slice[self.median_index])
                    / IndicatorValue::from(2.0),
            )
        } else {
            // For an odd number of elements, return the middle element
            Some(self.slice[self.median_index])
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let mut last_output = None;
        for &value in input {
            last_output = self.next(value);
        }
        last_output
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
        self.slice.clear();
    }
}
