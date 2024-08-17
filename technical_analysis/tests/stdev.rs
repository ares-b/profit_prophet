#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, StandardDeviation};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_stdev_next() {
        let mut stdev = StandardDeviation::new(3);
        assert_eq!((stdev.next(IndicatorValue::from(1.0)).to_f64() * 100.00).round(2) / 100.0, 0.58);
        assert_eq!(stdev.next(IndicatorValue::from(2.0)).to_f64(), 1.0);
        assert_eq!(stdev.next(IndicatorValue::from(3.0)).to_f64(), 1.0);
        assert_eq!(stdev.next(IndicatorValue::from(4.0)).to_f64(), 1.0);
    }

    #[test]
    fn test_stdev_next_chunk() {
        let mut stdev = StandardDeviation::new(3);
        let result = stdev.next_chunk(&[
            IndicatorValue::from(1.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(3.0),
            IndicatorValue::from(4.0),
        ]);
        assert_eq!(result.to_f64(), 1.0);
    }

    #[test]
    fn test_stdev_reset() {
        let mut stdev = StandardDeviation::new(3);
        stdev.next(IndicatorValue::from(1.0));
        stdev.next(IndicatorValue::from(2.0));
        stdev.reset();
        assert_eq!((stdev.next(IndicatorValue::from(3.0)).to_f64() * 100.00).round(2) / 100.0, 1.73);
    }

    #[test]
    fn test_stdev_with_large_data() {
        let mut stdev = StandardDeviation::new(100);
        let data: Vec<IndicatorValue> = (1..=100).map(|x| IndicatorValue::from(x as f64)).collect();
        let result = (stdev.next_chunk(&data) * 100.0.into()).round(2) / 100.0.into();
        assert_eq!(result.to_f64(), 29.01);
    }

    #[test]
    fn test_stdev_all_same_values() {
        let mut stdev = StandardDeviation::new(3);
        let result = (stdev.next_chunk(&[
            IndicatorValue::from(2.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(2.0),
        ]) * 100.00.into()).round(2) / 100.00.into();

        assert_eq!(result.to_f64(), 0.0);
    }

    #[test]
    fn test_stdev_with_zeros() {
        let mut stdev = StandardDeviation::new(3);
        let result = stdev.next_chunk(&[
            IndicatorValue::from(0.0),
            IndicatorValue::from(0.0),
            IndicatorValue::from(0.0),
            IndicatorValue::from(0.0),
        ]);
        assert_eq!(result.to_f64(), 0.0);
    }
}
