use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::str::FromStr;
use std::cmp::Ordering;
use std::iter::Sum;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct IndicatorValue(Decimal);

impl IndicatorValue {

    #[inline]
    pub fn max() -> Self {
        Self(Decimal::MAX)
    }

    #[inline]
    pub fn min() -> Self {
        Self(Decimal::MIN)
    }

    #[inline]
    pub fn value(&self) -> Decimal {
        self.0
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        self.0.to_f64().unwrap_or_else(|| panic!("Failed to convert Decimal to f64"))
    }
    
    #[inline]
    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt().unwrap_or_else(|| panic!("Failed to compute sqrt")))
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline]
    pub fn round_dp(&self, dp: u32) -> Self {
        Self(self.0.round_dp(dp))
    }

    #[inline]
    pub fn recip(&self) -> Self {
        Self(Decimal::from_f32(1.00).unwrap() / self.0)
    }
    
}

impl From<f64> for IndicatorValue {
    #[inline]
    fn from(value: f64) -> Self {
        Self(Decimal::from_f64(value).unwrap_or_else(|| panic!("Failed to convert f64 to Decimal")))
    }
}

impl From<&str> for IndicatorValue {
    #[inline]
    fn from(value: &str) -> Self {
        Self(Decimal::from_str(value).unwrap_or_else(|_| panic!("Invalid number format")))
    }
}

impl From<usize> for IndicatorValue {
    #[inline]
    fn from(value: usize) -> Self {
        Self(Decimal::from_usize(value).unwrap_or_else(|| panic!("Failed to convert usize to Decimal")))
    }
}

impl From<u64> for IndicatorValue {
    #[inline]
    fn from(value: u64) -> Self {
        Self(Decimal::from_u64(value).unwrap_or_else(|| panic!("Failed to convert u64 to Decimal")))
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
        self.0.cmp(&other.0)
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
        Self(-self.0)
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
