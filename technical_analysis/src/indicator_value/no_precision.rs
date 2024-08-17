use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::iter::Sum;
use std::cmp::Ordering;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct IndicatorValue(f64);

impl IndicatorValue {

    #[inline]
    pub fn max() -> Self {
        Self(f64::MAX)
    }

    #[inline]
    pub fn min() -> Self {
        Self(f64::MIN)
    }

    #[inline]
    pub fn value(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline]
    pub fn round_dp(&self, dp: u32) -> Self {
        let factor = 10f64.powi(dp as i32);
        let scaled_value = self.0 * factor;
        let integer_part = scaled_value.trunc();

        let rounded = if (scaled_value - integer_part).abs() == 0.5 {
            // Check if the integer part is even
            if integer_part % 2.0 == 0.0 {
                integer_part // If even, return the integer part
            } else {
                integer_part + scaled_value.signum() // If odd, round away from zero
            }
        } else {
            scaled_value.round() // Regular rounding if not exactly 0.5
        };

        Self(rounded / factor)
    }

    #[inline]
    pub fn recip(&self) -> Self {
        Self(self.0.recip())
    }
    
}

impl From<f64> for IndicatorValue {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<&str> for IndicatorValue {
    #[inline]
    fn from(value: &str) -> Self {
        Self(value.parse::<f64>().unwrap())
    }
}

impl From<usize> for IndicatorValue {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl From<u64> for IndicatorValue {
    #[inline]
    fn from(value: u64) -> Self {
        IndicatorValue::from(value as f64)
    }
}

impl PartialEq for IndicatorValue {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IndicatorValue {}

impl PartialOrd for IndicatorValue {

    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
    
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }
}

impl Ord for IndicatorValue {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl Sum for IndicatorValue {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(IndicatorValue::from(0.0), Add::add)
    }
}

impl Neg for IndicatorValue {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from(-self.0)
    }
}

impl Add for IndicatorValue {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for IndicatorValue {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul for IndicatorValue {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl Div for IndicatorValue {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

impl AddAssign for IndicatorValue {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl SubAssign for IndicatorValue {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl MulAssign for IndicatorValue {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

impl DivAssign for IndicatorValue {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}
