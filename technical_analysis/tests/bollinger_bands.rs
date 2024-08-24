#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, BollingerBands};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_bollinger_bands_next() {
        let mut bb = BollingerBands::new(20, 2);
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 
            22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63,
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for price in prices {
            bb.next(price);
        }

        let result = bb.next(IndicatorValue::from(23.82)).unwrap();
        assert_eq!(result.upper_band.round_dp(2), IndicatorValue::from(24.30));
        assert_eq!(result.lower_band.round_dp(2), IndicatorValue::from(21.28));
    }

    #[test]
    fn test_bollinger_bands_empty_input() {
        let mut bb = BollingerBands::new(20, 2);
        let result = bb.next(IndicatorValue::from(0.0));
        assert_eq!(result, None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bollinger_bands_single_input() {
        let mut bb = BollingerBands::new(20, 2);
        let result = bb.next(IndicatorValue::from(23.82));
        assert_eq!(result, None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bollinger_bands_with_constant_prices() {
        let mut bb = BollingerBands::new(20, 2);
        let prices = vec![IndicatorValue::from(50.0); 20];

        let result = bb.next_chunk(&prices).unwrap();
        assert_eq!(result.upper_band, IndicatorValue::from(50.0 + 2.0 * 0.0));
        assert_eq!(result.lower_band, IndicatorValue::from(50.0 - 2.0 * 0.0));
    }

    #[test]
    fn test_bollinger_bands_with_increasing_prices() {
        let mut bb = BollingerBands::new(20, 2);
        let prices: Vec<IndicatorValue> = (1..=20).map(|x| IndicatorValue::from(x as f64)).collect();

        let result = bb.next_chunk(&prices).unwrap();
        assert_eq!(result.upper_band.round_dp(2), IndicatorValue::from(22.33));
        assert_eq!(result.lower_band.round_dp(2), IndicatorValue::from(-1.33));
    }

    #[test]
    fn test_bollinger_bands_with_decreasing_prices() {
        let mut bb = BollingerBands::new(20, 2);
        let prices: Vec<IndicatorValue> = (1..=20).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let result = bb.next_chunk(&prices).unwrap();
        assert_eq!(result.upper_band.round_dp(2), IndicatorValue::from(22.33));
        assert_eq!(result.lower_band.round_dp(2), IndicatorValue::from(-1.33));
    }

    #[test]
    fn test_bollinger_bands_reset() {
        let mut bb = BollingerBands::new(20, 2);
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 
            22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for price in prices {
            bb.next(price);
        }

        bb.reset();
        let result = bb.next(IndicatorValue::from(23.82));
        assert_eq!(result, None);
        assert_eq!(result, None);
    }
}
