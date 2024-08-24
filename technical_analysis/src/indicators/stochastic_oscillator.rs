use crate::indicators::Indicator;
use crate::indicators::{High, Low, SimpleMovingAverage};
use crate::IndicatorValue;

pub struct StochasticOscillator {
    high: High,
    low: Low,
    d_sma: SimpleMovingAverage,
    last_k: IndicatorValue,
}

#[derive(Debug)]
pub struct StochasticOutput {
    pub k: IndicatorValue,
    pub d: IndicatorValue,
}

impl StochasticOscillator {
    #[inline]
    pub fn new(period: usize, d_period: usize) -> Self {
        StochasticOscillator {
            high: High::new(period),
            low: Low::new(period),
            d_sma: SimpleMovingAverage::new(d_period),
            last_k: 50.0.into(),
        }
    }
}

impl Default for StochasticOscillator {
    fn default() -> Self {
        StochasticOscillator::new(14, 3)
    }
}

impl Indicator for StochasticOscillator {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = Option<StochasticOutput>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;

        let highest_high = self.high.next(high);
        let lowest_low = self.low.next(low);
        
        match (highest_high, lowest_low) {
            (Some(high), Some(low)) => {
                let range = high.high_value - low.low_value;
                let k = if range > 0.0.into() {
                    (close - low.low_value) / range * 100.0.into()
                } else {
                    self.last_k 
                };
                let d = self.d_sma.next(k);
                self.last_k = k;
                
                match d {
                    Some(d) => Some(StochasticOutput { k, d }),
                    None => None
                }
            },
            _ => None
        }

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
        self.high.reset();
        self.low.reset();
        self.last_k = 50.0.into();
    }
}
