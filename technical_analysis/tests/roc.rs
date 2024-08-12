#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, RateOfChange};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_roc_next() {
        let mut roc = RateOfChange::new(3);
        assert_eq!(roc.next(IndicatorValue::from(1.0)).to_f64(), 0.0);
        assert_eq!(roc.next(IndicatorValue::from(2.0)).to_f64(), 100.0);
        assert_eq!(roc.next(IndicatorValue::from(3.0)).to_f64(), 50.0);
        assert_eq!(roc.next(IndicatorValue::from(4.0)).to_f64(), 33.33);
    }

    #[test]
    fn test_roc_next_chunk() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[
            IndicatorValue::from(1.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(3.0),
            IndicatorValue::from(4.0),
        ]);
        assert_eq!(result.to_f64(), 33.33);
    }

    #[test]
    fn test_roc_reset() {
        let mut roc = RateOfChange::new(3);
        roc.next(IndicatorValue::from(1.0));
        roc.next(IndicatorValue::from(2.0));
        roc.reset();
        assert_eq!(roc.next(IndicatorValue::from(3.0)).to_f64(), 0.0);
    }

    #[test]
    fn test_roc_with_large_data() {
        let mut roc = RateOfChange::new(100);
        let data: Vec<IndicatorValue> = (1..=1000).map(|x| IndicatorValue::from(x as f64)).collect();
        let result = roc.next_chunk(&data);
        assert!(result.to_f64() > 0.0 && result.to_f64() < 100.0); // ROC should be between 0% and 100% depending on data
    }

    #[test]
    fn test_roc_all_same_values() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[
            IndicatorValue::from(2.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(2.0),
        ]);
        assert_eq!(result.to_f64(), 0.0);
    }

    #[test]
    fn test_roc_with_zeros() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[
            IndicatorValue::from(0.0),
            IndicatorValue::from(0.0),
            IndicatorValue::from(0.0),
            IndicatorValue::from(0.0),
        ]);
        assert_eq!(result.to_f64(), 0.0);
    }

    #[test]
    fn test_roc_with_very_small_values() {
        let mut roc = RateOfChange::new(3);
        let result = roc.next_chunk(&[
            IndicatorValue::from(1.0),
            IndicatorValue::from(2.0),
            IndicatorValue::from(3.0),
            IndicatorValue::from(4.0),
        ]);
        assert_eq!(result.to_f64(), 33.33);
    }
}
