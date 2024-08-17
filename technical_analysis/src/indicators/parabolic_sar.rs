use crate::indicators::Indicator;
use crate::IndicatorValue;

pub struct ParabolicSAR {
    acceleration_factor: IndicatorValue,
    max_acceleration_factor: IndicatorValue,
    current_sar: IndicatorValue,
    extreme_point: IndicatorValue,
    is_long: bool,
    initial_acceleration_factor: IndicatorValue,
}

impl ParabolicSAR {
    #[inline]
    pub fn new(acceleration_factor: f64, max_acceleration_factor: f64) -> Self {
        let acceleration_factor = acceleration_factor.into();
        ParabolicSAR {
            acceleration_factor,
            max_acceleration_factor: max_acceleration_factor.into(),
            current_sar: 0.0.into(),
            extreme_point: 0.0.into(),
            is_long: true,
            initial_acceleration_factor: acceleration_factor,
        }
    }

    #[inline]
    fn switch_to_short(&mut self, low: IndicatorValue) {
        self.is_long = false;
        self.current_sar = self.extreme_point;
        self.extreme_point = low;
        self.acceleration_factor = self.initial_acceleration_factor;
    }

    #[inline]
    fn switch_to_long(&mut self, high: IndicatorValue) {
        self.is_long = true;
        self.current_sar = self.extreme_point;
        self.extreme_point = high;
        self.acceleration_factor = self.initial_acceleration_factor;
    }

    #[inline]
    fn update_long(&mut self, high: IndicatorValue) {
        if high > self.extreme_point {
            self.extreme_point = high;
            self.acceleration_factor = (self.acceleration_factor + self.initial_acceleration_factor)
                .min(self.max_acceleration_factor);
        }
        self.current_sar += self.acceleration_factor * (self.extreme_point - self.current_sar);
    }

    #[inline]
    fn update_short(&mut self, low: IndicatorValue) {
        if low < self.extreme_point {
            self.extreme_point = low;
            self.acceleration_factor = (self.acceleration_factor + self.initial_acceleration_factor)
                .min(self.max_acceleration_factor);
        }
        self.current_sar -= self.acceleration_factor * (self.current_sar - self.extreme_point);
    }
}

impl Default for ParabolicSAR {
    fn default() -> Self {
        ParabolicSAR::new(0.02, 0.2)
    }
}

impl Indicator for ParabolicSAR {
    type Input = (IndicatorValue, IndicatorValue);
    type Output = IndicatorValue;

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low) = input;

        if self.is_long {
            if low < self.current_sar {
                self.switch_to_short(low);
            } else {
                self.update_long(high);
            }
        } else {
            if high > self.current_sar {
                self.switch_to_long(high);
            } else {
                self.update_short(low);
            }
        }

        self.current_sar
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().fold(self.current_sar, |_, &value| self.next(value))
    }

    #[inline]
    fn reset(&mut self) {
        self.current_sar = 0.0.into();
        self.extreme_point = 0.0.into();
        self.is_long = true;
        self.acceleration_factor = self.initial_acceleration_factor;
    }
}
