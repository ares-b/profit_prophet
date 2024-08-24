use crate::indicators::Indicator;
use crate::indicators::{SimpleMovingAverage, StandardDeviation};
use crate::IndicatorValue;

pub struct BollingerBands {
    multiplier: IndicatorValue,
    sma: SimpleMovingAverage,
    std_dev: StandardDeviation,
}

#[derive(Debug, PartialEq)]
pub struct BollingerBandsOutput {
    pub upper_band: IndicatorValue,
    pub lower_band: IndicatorValue,
}

impl Default for BollingerBandsOutput {
    fn default() -> Self {
        Self { upper_band: 0.0.into(), lower_band: 0.0.into() }
    }
}

impl Default for BollingerBands {
    fn default() -> Self {
        Self::new(20, 2)
    }
}

impl BollingerBands {
    #[inline]
    pub fn new(period: usize, multiplier: usize) -> Self {
        BollingerBands {
            multiplier: multiplier.into(),
            sma: SimpleMovingAverage::new(period),
            std_dev: StandardDeviation::new(period),
        }
    }
}

impl Indicator for BollingerBands {
    type Output = Option<BollingerBandsOutput>;
    type Input = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let sma_value = self.sma.next(input);
        let std_dev_value = self.std_dev.next(input);

        match (sma_value, std_dev_value) {
            (Some(sma), Some(stdev)) => {
                let offset = self.multiplier * stdev;
                Some(BollingerBandsOutput {
                    upper_band: sma + offset,
                    lower_band: sma - offset,
                })
            },
            _ => None
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
        self.sma.reset();
        self.std_dev.reset();
    }
}
