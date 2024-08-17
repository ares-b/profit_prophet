#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, OnBalanceVolume};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_obv_initial() {
        let mut obv = OnBalanceVolume::default();
        let result = obv.next((IndicatorValue::from(100.0), IndicatorValue::from(1000.0)));
        assert_eq!(result.to_f64(), 1000.0); // Initial OBV should equal the first volume if it's the first price
    }

    #[test]
    fn test_obv_next() {
        let mut obv = OnBalanceVolume::default();
        obv.next((IndicatorValue::from(100.0), IndicatorValue::from(1000.0)));
        let result = obv.next((IndicatorValue::from(102.0), IndicatorValue::from(1200.0)));
        assert_eq!(result.to_f64(), 2200.0); // OBV should increase by the volume if price increases

        let result = obv.next((IndicatorValue::from(101.0), IndicatorValue::from(1500.0)));
        assert_eq!(result.to_f64(), 3700.0); // OBV should increase by the volume if price decreases
    }

    #[test]
    fn test_obv_next_with_price_decrease() {
        let mut obv = OnBalanceVolume::default();
        obv.next((IndicatorValue::from(100.0), IndicatorValue::from(1000.0)));
        let result = obv.next((IndicatorValue::from(98.0), IndicatorValue::from(1500.0)));
        assert_eq!(result.to_f64(), -500.0); // OBV should decrease by the volume if price decreases

        let result = obv.next((IndicatorValue::from(95.0), IndicatorValue::from(2000.0)));
        assert_eq!(result.to_f64(), -2500.0); // OBV should decrease by the volume if price decreases again
    }

    #[test]
    fn test_obv_next_with_constant_price() {
        let mut obv = OnBalanceVolume::default();
        obv.next((IndicatorValue::from(100.0), IndicatorValue::from(1000.0)));
        let result = obv.next((IndicatorValue::from(100.0), IndicatorValue::from(2000.0)));
        assert_eq!(result.to_f64(), 1000.0); // OBV should remain unchanged if the price is constant
    }

    #[test]
    fn test_obv_with_chunk() {
        let mut obv = OnBalanceVolume::default();
        let data = vec![
            (100.0, 1000.0), // Price increases
            (98.0, 1500.0),  // Price decreases
            (102.0, 2000.0), // Price increases
        ]
        .into_iter()
        .map(|(p, v)| (IndicatorValue::from(p), IndicatorValue::from(v)))
        .collect::<Vec<_>>();

        let result = obv.next_chunk(&data);
        assert_eq!(result.to_f64(), 1500.0); // OBV should reflect cumulative changes
    }

    #[test]
    fn test_obv_reset() {
        let mut obv = OnBalanceVolume::default();
        obv.next((IndicatorValue::from(100.0), IndicatorValue::from(1000.0)));
        obv.next((IndicatorValue::from(102.0), IndicatorValue::from(2000.0)));

        obv.reset();
        let result = obv.next((IndicatorValue::from(104.0), IndicatorValue::from(1500.0)));
        assert_eq!(result.to_f64(), 1500.0); // After reset, OBV should start fresh
    }

    #[test]
    fn test_obv_with_no_volume() {
        let mut obv = OnBalanceVolume::default();
        obv.next((IndicatorValue::from(100.0), IndicatorValue::from(0.0)));
        let result = obv.next((IndicatorValue::from(102.0), IndicatorValue::from(0.0)));
        assert_eq!(result.to_f64(), 0.0); // OBV should remain zero if there's no volume
    }

    #[test]
    fn test_obv_with_large_volume() {
        let mut obv = OnBalanceVolume::default();
        obv.next((IndicatorValue::from(100.0), IndicatorValue::from(1_000_000.0)));
        let result = obv.next((IndicatorValue::from(102.0), IndicatorValue::from(2_000_000.0)));
        assert_eq!(result.to_f64(), 3_000_000.0); // OBV should handle large volumes correctly
    }

    #[test]
    fn test_obv_with_increasing_prices_and_volumes() {
        let mut obv = OnBalanceVolume::default();
        let data = vec![
            (100.0, 1000.0),
            (101.0, 2000.0),
            (102.0, 3000.0),
            (103.0, 4000.0),
        ]
        .into_iter()
        .map(|(p, v)| (IndicatorValue::from(p), IndicatorValue::from(v)))
        .collect::<Vec<_>>();

        let result = obv.next_chunk(&data);
        assert_eq!(result.to_f64(), 10_000.0); // OBV should accumulate correctly with increasing prices and volumes
    }

    #[test]
    fn test_obv_with_decreasing_prices_and_volumes() {
        let mut obv = OnBalanceVolume::default();
        let data = vec![
            (103.0, 4000.0),
            (102.0, 3000.0),
            (101.0, 2000.0),
            (100.0, 1000.0),
        ]
        .into_iter()
        .map(|(p, v)| (IndicatorValue::from(p), IndicatorValue::from(v)))
        .collect::<Vec<_>>();

        let result = obv.next_chunk(&data);
        assert_eq!(result.to_f64(), -10_000.0); // OBV should decrease correctly with decreasing prices and volumes
    }
}
