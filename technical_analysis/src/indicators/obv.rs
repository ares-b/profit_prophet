use crate::indicators::Indicator;
use crate::IndicatorValue;

pub struct OnBalanceVolume {
    prev_close: Option<IndicatorValue>,
    obv: IndicatorValue,
}

impl OnBalanceVolume {
    #[inline]
    pub fn new() -> Self {
        OnBalanceVolume {
            prev_close: None,
            obv: 0.0.into(),
        }
    }
}

impl Default for OnBalanceVolume {
    fn default() -> Self {
        OnBalanceVolume::new()
    }
}

impl Indicator for OnBalanceVolume {
    type Input = (IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (close, volume) = input;

        match self.prev_close {
            Some(prev_close) if close > prev_close => self.obv += volume,
            Some(prev_close) if close < prev_close => self.obv -= volume,
            _ => {},
        }

        self.prev_close = Some(close);
        self.obv
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(0.0.into(), |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.prev_close = None;
        self.obv = 0.0.into();
    }
}