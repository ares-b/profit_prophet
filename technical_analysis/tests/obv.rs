#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, OnBalanceVolume};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_obv_typical_case() {
        let mut obv = OnBalanceVolume::new();
        let prices_and_volumes = vec![
            (IndicatorValue::from(44.34), IndicatorValue::from(100.0)),  // Initial value, OBV = 0
            (IndicatorValue::from(44.09), IndicatorValue::from(120.0)),  // Price down, OBV = -120
            (IndicatorValue::from(44.15), IndicatorValue::from(130.0)),  // Price up, OBV = 10
            (IndicatorValue::from(43.61), IndicatorValue::from(90.0)),   // Price down, OBV = -80
            (IndicatorValue::from(44.33), IndicatorValue::from(150.0)),  // Price up, OBV = 70
        ];

        let result = obv.next_chunk(&prices_and_volumes);
        assert_eq!(result, IndicatorValue::from(70.0));
    }

    #[test]
    fn test_obv_constant_prices() {
        let mut obv = OnBalanceVolume::new();
        let prices_and_volumes = vec![
            (IndicatorValue::from(44.34), IndicatorValue::from(100.0)),  // Initial value, OBV = 0
            (IndicatorValue::from(44.34), IndicatorValue::from(120.0)),  // Price unchanged, OBV = 0
            (IndicatorValue::from(44.34), IndicatorValue::from(130.0)),  // Price unchanged, OBV = 0
        ];

        let result = obv.next_chunk(&prices_and_volumes);
        assert_eq!(result, IndicatorValue::from(0.0));
    }

    #[test]
    fn test_obv_all_upward() {
        let mut obv = OnBalanceVolume::new();
        let prices_and_volumes = vec![
            (IndicatorValue::from(44.34), IndicatorValue::from(100.0)),  // Initial value, OBV = 0
            (IndicatorValue::from(44.50), IndicatorValue::from(120.0)),  // Price up, OBV = 120
            (IndicatorValue::from(44.75), IndicatorValue::from(130.0)),  // Price up, OBV = 250
            (IndicatorValue::from(45.00), IndicatorValue::from(90.0)),   // Price up, OBV = 340
            (IndicatorValue::from(45.25), IndicatorValue::from(150.0)),  // Price up, OBV = 490
        ];

        let result = obv.next_chunk(&prices_and_volumes);
        assert_eq!(result, IndicatorValue::from(490.0));
    }

    #[test]
    fn test_obv_all_downward() {
        let mut obv = OnBalanceVolume::new();
        let prices_and_volumes = vec![
            (IndicatorValue::from(45.25), IndicatorValue::from(150.0)),  // Initial value, OBV = 0
            (IndicatorValue::from(45.00), IndicatorValue::from(120.0)),  // Price down, OBV = -120
            (IndicatorValue::from(44.75), IndicatorValue::from(130.0)),  // Price down, OBV = -250
            (IndicatorValue::from(44.50), IndicatorValue::from(90.0)),   // Price down, OBV = -340
            (IndicatorValue::from(44.25), IndicatorValue::from(150.0)),  // Price down, OBV = -490
        ];

        let result = obv.next_chunk(&prices_and_volumes);
        assert_eq!(result, IndicatorValue::from(-490.0));
    }

    #[test]
    fn test_obv_reset_behavior() {
        let mut obv = OnBalanceVolume::new();
        let prices_and_volumes = vec![
            (IndicatorValue::from(44.34), IndicatorValue::from(100.0)),  // Initial value, OBV = 0
            (IndicatorValue::from(44.09), IndicatorValue::from(120.0)),  // Price down, OBV = -120
        ];

        obv.next_chunk(&prices_and_volumes);
        obv.reset();

        let new_prices_and_volumes = vec![
            (IndicatorValue::from(45.00), IndicatorValue::from(100.0)),  // After reset, OBV = 0
            (IndicatorValue::from(45.25), IndicatorValue::from(150.0)),  // Price up, OBV = 150
        ];

        let result = obv.next_chunk(&new_prices_and_volumes);
        assert_eq!(result, IndicatorValue::from(150.0));
    }

    #[test]
    fn test_obv_mixed_movement() {
        let mut obv = OnBalanceVolume::new();
        let prices_and_volumes = vec![
            (IndicatorValue::from(44.34), IndicatorValue::from(100.0)),  // Initial value, OBV = 0
            (IndicatorValue::from(44.50), IndicatorValue::from(120.0)),  // Price up, OBV = 120
            (IndicatorValue::from(44.25), IndicatorValue::from(130.0)),  // Price down, OBV = -10
            (IndicatorValue::from(44.75), IndicatorValue::from(90.0)),   // Price up, OBV = 80
            (IndicatorValue::from(44.50), IndicatorValue::from(150.0)),  // Price down, OBV = -70
        ];

        let result = obv.next_chunk(&prices_and_volumes);
        assert_eq!(result, IndicatorValue::from(-70.0));
    }

    #[test]
    fn test_obv_single_data_point() {
        let mut obv = OnBalanceVolume::new();
        let prices_and_volumes = vec![
            (IndicatorValue::from(44.34), IndicatorValue::from(100.0)),  // Initial value, OBV = 0
        ];

        let result = obv.next_chunk(&prices_and_volumes);
        assert_eq!(result, IndicatorValue::from(0.0));
    }
}
