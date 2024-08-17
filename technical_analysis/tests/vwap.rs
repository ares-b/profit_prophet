#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, VolumeWeightedAveragePrice};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_vwap_next() {
        let mut vwap = VolumeWeightedAveragePrice::new();
        let data = vec![
            (22.27, 22.19, 22.25, 2000.0), (22.29, 22.21, 22.28, 1500.0),
            (22.31, 22.23, 22.30, 1800.0), (22.33, 22.25, 22.32, 1700.0),
            (22.35, 22.27, 22.34, 1600.0), (22.37, 22.29, 22.36, 1900.0),
        ].into_iter().map(|(h, l, c, v)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c), IndicatorValue::from(v))).collect::<Vec<_>>();

        for value in data {
            vwap.next(value);
        }

        let result = vwap.next((IndicatorValue::from(22.39), IndicatorValue::from(22.31), IndicatorValue::from(22.38), IndicatorValue::from(2000.0)));
        assert!(result.to_f64() > 0.0); // VWAP should be calculated
    }

    #[test]
    fn test_vwap_empty_input() {
        let mut vwap = VolumeWeightedAveragePrice::new();
        let result = vwap.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result.to_f64(), 0.0); // VWAP with no data should be zero
    }

    #[test]
    fn test_vwap_single_input() {
        let mut vwap = VolumeWeightedAveragePrice::new();
        let result = vwap.next((IndicatorValue::from(22.39), IndicatorValue::from(22.31), IndicatorValue::from(22.38), IndicatorValue::from(2000.0)));
        assert_eq!(result.to_f64(), 22.38); // VWAP with single input should equal the typical price
    }

    #[test]
    fn test_vwap_with_constant_prices() {
        let mut vwap = VolumeWeightedAveragePrice::new();
        let data = vec![(22.55, 21.55, 22.55, 2000.0); 20]
            .into_iter()
            .map(|(h, l, c, v)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c), IndicatorValue::from(v)))
            .collect::<Vec<_>>();

        let result = vwap.next_chunk(&data);
        assert_eq!(result.to_f64(), 22.55); // VWAP with constant prices should equal the typical price
    }

    #[test]
    fn test_vwap_reset() {
        let mut vwap = VolumeWeightedAveragePrice::new();
        let data = vec![
            (22.27, 22.19, 22.25, 2000.0), (22.29, 22.21, 22.28, 1500.0),
        ].into_iter().map(|(h, l, c, v)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c), IndicatorValue::from(v))).collect::<Vec<_>>();

        for value in data {
            vwap.next(value);
        }

        vwap.reset();
        let result = vwap.next((IndicatorValue::from(22.39), IndicatorValue::from(22.31), IndicatorValue::from(22.38), IndicatorValue::from(2000.0)));
        assert_eq!(result.to_f64(), 22.38); // After reset, it should start fresh
    }
}
