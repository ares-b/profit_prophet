use crate::indicators::Indicator;
use crate::CircularBuffer;
use crate::IndicatorValue;

pub struct MeanAbsDev {
    buffer: CircularBuffer,
    sum: IndicatorValue,
    mean: IndicatorValue,
    reciprocal_period: IndicatorValue,
}

impl MeanAbsDev {
    #[inline]
    pub fn new(period: usize) -> Self {
        let reciprocal_period = 1.0 / period as f64;
        MeanAbsDev {
            buffer: CircularBuffer::new(period),
            sum: 0.0.into(),
            mean: 0.0.into(),
            reciprocal_period: reciprocal_period.into(),
        }
    }

    #[inline]
    fn update_mean(&mut self, new_value: IndicatorValue, old_value: IndicatorValue) {
        self.sum += new_value - old_value;
        self.mean = self.sum * self.reciprocal_period;
    }

    #[inline]
    fn calculate_mad(&self) -> IndicatorValue {
        self.buffer.iter().map(|&value| (value - self.mean).abs()).sum::<IndicatorValue>() * self.reciprocal_period
    }
}

impl Default for MeanAbsDev {
    fn default() -> Self {
        MeanAbsDev::new(20)
    }
}

impl Indicator for MeanAbsDev {
    type Input = IndicatorValue;
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let old_value = if self.buffer.is_full() {
            self.buffer.push(input).unwrap() // return the replaced value
        } else {
            self.buffer.push(input).unwrap_or(0.0.into()) // no old value to replace
        };

        if self.buffer.is_full() {
            self.update_mean(input, old_value);
            self.calculate_mad()
        } else {
            self.sum += input;
            self.mean = self.sum / (self.buffer.len() as f64).into();
            0.0.into()
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.buffer.clear();
        self.sum = 0.0.into();
        self.mean = 0.0.into();
    }
}
