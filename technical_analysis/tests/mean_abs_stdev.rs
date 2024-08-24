#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, MeanAbsDev};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_mad_with_empty_buffer() {
        let mut mad = MeanAbsDev::new(5);
        assert_eq!(mad.next(0.0.into()), None);
    }

    #[test]
    fn test_mad_with_partial_buffer() {
        let mut mad = MeanAbsDev::new(3);
        mad.next(1.0.into());
        mad.next(2.0.into());
        let result = mad.next(3.0.into());
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(0.67));
    }

    #[test]
    fn test_mad_with_full_buffer() {
        let mut mad = MeanAbsDev::new(3);
        mad.next(1.0.into());
        mad.next(2.0.into());
        mad.next(3.0.into());
        let result = mad.next(4.0.into());
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(0.67));
    }

    #[test]
    fn test_mad_reset() {
        let mut mad = MeanAbsDev::new(3);
        mad.next(1.0.into());
        mad.next(2.0.into());
        mad.reset();
        let result = mad.next(3.0.into());
        assert_eq!(result, None);
    }

    #[test]
    fn test_mad_large_input() {
        let mut mad = MeanAbsDev::new(5);
        mad.next(10.0.into());
        mad.next(20.0.into());
        mad.next(30.0.into());
        mad.next(40.0.into());
        mad.next(50.0.into());
        let result = mad.next(60.0.into());
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(12.0));
    }
}
