use crate::IndicatorValue;

#[derive(Clone, Copy)]
pub struct OHLCV {
    pub open: IndicatorValue,
    pub high: IndicatorValue,
    pub low: IndicatorValue,
    pub close: IndicatorValue,
    pub volume: IndicatorValue,
}
