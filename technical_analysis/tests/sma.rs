#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, SimpleMovingAverage};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_sma_next() {
        let mut sma = SimpleMovingAverage::new(3);
        assert_eq!(sma.next(IndicatorValue::from(1.0)).round_dp(2), IndicatorValue::from(0.33));
        assert_eq!(sma.next(IndicatorValue::from(2.0)).round_dp(2), IndicatorValue::from(1.0));
        assert_eq!(sma.next(IndicatorValue::from(3.0)).round_dp(2), IndicatorValue::from(2.0));
        assert_eq!(sma.next(IndicatorValue::from(4.0)).round_dp(2), IndicatorValue::from(3.0));
    }

    #[test]
    fn test_sma_next_chunk() {
        let mut sma = SimpleMovingAverage::new(3);
        let result = sma.next_chunk(&[
            IndicatorValue::from(1.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(3.0),
            IndicatorValue::from(4.0),
        ]);
        assert_eq!(result.round_dp(2), IndicatorValue::from(3.0));
    }

    #[test]
    fn test_sma_reset() {
        let mut sma = SimpleMovingAverage::new(3);
        sma.next(IndicatorValue::from(1.0));
        sma.next(IndicatorValue::from(2.0));
        sma.reset();
        assert_eq!(sma.next(IndicatorValue::from(3.0)).round_dp(2), IndicatorValue::from(1.0));
    }

    #[test]
    fn test_sma_with_large_data() {
        let mut sma = SimpleMovingAverage::new(100);
        let data: Vec<IndicatorValue> = (1..=1000).map(|x| IndicatorValue::from(x as f64)).collect();
        let result = sma.next_chunk(&data);
        assert_eq!(result.round_dp(2), IndicatorValue::from(950.5));
    }

    #[test]
    fn test_sma_all_same_values() {
        let mut sma = SimpleMovingAverage::new(3);
        assert_eq!(
            sma.next_chunk(&[IndicatorValue::from(2.0), IndicatorValue::from(2.0), IndicatorValue::from(2.0), IndicatorValue::from(2.0)]).round_dp(2),
            IndicatorValue::from(2.0)
        );
    }

    #[test]
    fn test_sma_with_zeros() {
        let mut sma = SimpleMovingAverage::new(3);
        assert_eq!(
            sma.next_chunk(&[IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)]).round_dp(2),
            IndicatorValue::from(0.0)
        );
    }
}
