use crate::indicators::Indicator;
use crate::IndicatorValue;

pub struct VolumeWeightedAveragePrice {
    cumulative_vp: IndicatorValue,
    cumulative_volume: IndicatorValue,
}

impl VolumeWeightedAveragePrice {
    #[inline]
    pub fn new() -> Self {
        VolumeWeightedAveragePrice {
            cumulative_vp: 0.0.into(),
            cumulative_volume: 0.0.into(),
        }
    }
}

impl Default for VolumeWeightedAveragePrice {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Indicator for VolumeWeightedAveragePrice {
    type Input = (IndicatorValue, IndicatorValue, IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low, close, volume) = input;

        let one_third: IndicatorValue = (1.0 / 3.0).into();

        let typical_price = (high + low + close) * one_third;

        self.cumulative_vp += typical_price * volume;
        self.cumulative_volume += volume;

        if self.cumulative_volume > 0.0.into() {
            self.cumulative_vp / self.cumulative_volume
        } else {
            0.0.into()
        }
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        let one_third: IndicatorValue = (1.0 / 3.0).into();

        for &(high, low, close, volume) in input.iter() {
            let typical_price = (high + low + close) * one_third;

            self.cumulative_vp += typical_price * volume;
            self.cumulative_volume += volume;
        }

        if self.cumulative_volume > 0.0.into() {
            self.cumulative_vp / self.cumulative_volume
        } else {
            0.0.into()
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.cumulative_vp = 0.0.into();
        self.cumulative_volume = 0.0.into();
    }
}
