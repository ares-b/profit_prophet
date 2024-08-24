use crate::IndicatorValue;
use crate::indicators::{Indicator, SimpleMovingAverage, MeanAbsDev};

pub struct CommodityChannelIndex {
    sma: SimpleMovingAverage,
    mean_abs_dev: MeanAbsDev,
    constant: IndicatorValue,
}

impl CommodityChannelIndex {
    #[inline]
    pub fn new(period: usize) -> Self {
        Self {
            sma: SimpleMovingAverage::new(period),
            mean_abs_dev: MeanAbsDev::new(period),
            constant: IndicatorValue::from(0.015),
        }
    }

}

impl Indicator for CommodityChannelIndex {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = Option<IndicatorValue>;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close) = input;
        let typical_price = (high + low + close) / IndicatorValue::from(3.0);
        
        let ma = self.sma.next(typical_price);
        let mean_deviation = self.mean_abs_dev.next(typical_price);
        
        match (ma, mean_deviation) {
           (Some(ma), Some(mean_deviation)) => {
                if mean_deviation == IndicatorValue::from(0.0) {
                    Some(IndicatorValue::from(0.0))
                } else {
                    Some((typical_price - ma) / (self.constant * mean_deviation))
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
        self.sma.reset();
        self.mean_abs_dev.reset();
    }
}
