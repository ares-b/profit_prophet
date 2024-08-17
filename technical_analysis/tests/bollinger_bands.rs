#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, BollingerBands};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_bollinger_bands_next() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 
            22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for price in prices {
            bb.next(price);
        }

        let result = bb.next(IndicatorValue::from(23.82));
        assert_eq!(IndicatorValue::from(22.41 + 2.0 * 0.52), result.upper_band); // Upper band example
        assert_eq!(IndicatorValue::from(22.41 - 2.0 * 0.52), result.lower_band); // Lower band example
    }

    #[test]
    fn test_bollinger_bands_empty_input() {
        let mut bb = BollingerBands::new(20, 2.0);
        let result = bb.next(IndicatorValue::from(0.0));
        assert_eq!(result.upper_band.to_f64(), 0.0); // Upper band should be zero with empty input
        assert_eq!(result.lower_band.to_f64(), 0.0); // Lower band should be zero with empty input
    }

    #[test]
    fn test_bollinger_bands_single_input() {
        let mut bb = BollingerBands::new(20, 2.0);
        let result = bb.next(IndicatorValue::from(23.82));
        assert_eq!(result.upper_band.to_f64(), 23.82); // With one value, upper and lower should match the input
        assert_eq!(result.lower_band.to_f64(), 23.82);
    }

    #[test]
    fn test_bollinger_bands_with_constant_prices() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices = vec![IndicatorValue::from(50.0); 20]; // Constant prices

        let result = bb.next_chunk(&prices);
        assert_eq!(IndicatorValue::from(50.0 + 2.0 * 0.0), result.upper_band); // Upper band with no variation
        assert_eq!(IndicatorValue::from(50.0 - 2.0 * 0.0), result.lower_band); // Lower band with no variation
    }

    #[test]
    fn test_bollinger_bands_with_increasing_prices() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices: Vec<IndicatorValue> = (1..=20).map(|x| IndicatorValue::from(x as f64)).collect();

        let result = bb.next_chunk(&prices);
        assert!(result.upper_band.to_f64() > 20.0); // Upper band should be above SMA
        assert!(result.lower_band.to_f64() < 20.0); // Lower band should be below SMA
    }

    #[test]
    fn test_bollinger_bands_with_decreasing_prices() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices: Vec<IndicatorValue> = (1..=20).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let result = bb.next_chunk(&prices);
        assert!(result.upper_band.to_f64() > result.lower_band.to_f64()); // Upper band should be above lower band
    }

    #[test]
    fn test_bollinger_bands_reset() {
        let mut bb = BollingerBands::new(20, 2.0);
        let prices = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 
            22.15, 22.39, 22.38, 22.61, 23.36, 24.05, 23.75, 23.83, 23.95, 23.63
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for price in prices {
            bb.next(price);
        }

        bb.reset();
        let result = bb.next(IndicatorValue::from(23.82));
        assert_eq!(IndicatorValue::from(23.82), result.upper_band); // After reset, it should start fresh
        assert_eq!(IndicatorValue::from(23.82), result.lower_band); // After reset, lower band should equal price
    }
}
