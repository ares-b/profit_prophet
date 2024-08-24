use crate::indicators::Indicator;
use crate::IndicatorValue;

pub struct ParabolicSAR {
    sar: IndicatorValue,
    ep: IndicatorValue,
    af: IndicatorValue,
    max_af: IndicatorValue,
    is_rising: bool,
    periods: usize,
    initialization_period: usize,
}

impl Default for ParabolicSAR {
    fn default() -> ParabolicSAR {
        ParabolicSAR::new(0.02, 0.2, 5)
    }
}

impl ParabolicSAR {
    #[inline]
    pub fn new(acceleration_factor: f64, max_acceleration_factor: f64, initialization_period: usize) -> ParabolicSAR {
        ParabolicSAR {
            sar: IndicatorValue::from(0.0),
            ep: IndicatorValue::from(0.0),
            af: IndicatorValue::from(acceleration_factor),
            max_af: IndicatorValue::from(max_acceleration_factor),
            is_rising: false,
            periods: 0,
            initialization_period,
        }
    }
}

impl Indicator for ParabolicSAR {
    type Output = Option<IndicatorValue>;
    type Input = (IndicatorValue, IndicatorValue);

    #[inline]
    fn next(&mut self, input: Self::Input) -> Self::Output {
        let (high, low) = input;

        if self.periods < self.initialization_period {
            self.periods += 1;

            if self.periods == 1 {
                self.is_rising = high > low;
                self.sar = if self.is_rising { low } else { high };
                self.ep = if self.is_rising { high } else { low };
            } else {
                if self.is_rising {
                    self.sar = self.sar.min(low);
                    self.ep = self.ep.max(high);
                } else {
                    self.sar = self.sar.max(high);
                    self.ep = self.ep.min(low);
                }
            }

            return None;
        }

        self.sar += self.af * (self.ep - self.sar);

        if self.is_rising {
            if low < self.sar {
                self.is_rising = false;
                self.sar = self.ep;
                self.ep = low;
                self.af = IndicatorValue::from(0.02);
            } else if high > self.ep {
                self.ep = high;
                self.af = (self.af + IndicatorValue::from(0.02)).min(self.max_af);
            }
        } else {
            if high > self.sar {
                self.is_rising = true;
                self.sar = self.ep;
                self.ep = high;
                self.af = IndicatorValue::from(0.02);
            } else if low < self.ep {
                self.ep = low;
                self.af = (self.af + IndicatorValue::from(0.02)).min(self.max_af);
            }
        }

        Some(self.sar)
    }

    #[inline]
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output {
        input.iter().map(|&(high, low)| self.next((high, low))).last().unwrap_or(None)
    }

    #[inline]
    fn reset(&mut self) {
        self.sar = IndicatorValue::from(0.0);
        self.ep = IndicatorValue::from(0.0);
        self.af = IndicatorValue::from(0.02);
        self.is_rising = false;
        self.periods = 0;
    }
}
