#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, MedianAbsoluteStandardDeviation};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_mad_basic_functionality() {
        let mut mad = MedianAbsoluteStandardDeviation::new(3);

        assert_eq!(mad.next(IndicatorValue::from(10.0)).round_dp(2), IndicatorValue::from(0.0));
        assert_eq!(mad.next(IndicatorValue::from(20.0)).round_dp(2), 5.0.into());
        assert_eq!(mad.next(IndicatorValue::from(30.0)).round_dp(2), 10.0.into());
        assert_eq!(mad.next(IndicatorValue::from(40.0)).round_dp(2), 10.0.into());
        assert_eq!(mad.next(IndicatorValue::from(50.0)).round_dp(2), 10.0.into());
    }

    #[test]
    fn test_mad_reset() {
        let mut mad = MedianAbsoluteStandardDeviation::new(3);

        mad.next(IndicatorValue::from(10.0));
        mad.next(IndicatorValue::from(20.0));
        mad.reset();

        assert_eq!(mad.next(IndicatorValue::from(10.0)).round_dp(2), IndicatorValue::from(0.0));
    }

    #[test]
    fn test_mad_single_value() {
        let mut mad = MedianAbsoluteStandardDeviation::new(3);
        assert_eq!(mad.next(IndicatorValue::from(10.0)).round_dp(2), IndicatorValue::from(0.0));
    }
}
