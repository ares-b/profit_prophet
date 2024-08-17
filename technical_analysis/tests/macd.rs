#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, MovingAverageConvergenceDivergence};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_macd_next() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            macd.next(price);
        }

        let result = macd.next(IndicatorValue::from(23.82));
        assert!(result.macd_value.to_f64() > 0.0); // MACD should be calculated
        assert!(result.signal_value.to_f64() > 0.0); // Signal should be calculated
        assert!(result.histogram_value.to_f64() > 0.0); // Histogram should be calculated
    }

    #[test]
    fn test_macd_empty_input() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let result = macd.next(IndicatorValue::from(0.0));
        assert_eq!(result.macd_value.to_f64(), 0.0); // MACD with no data should be zero
        assert_eq!(result.signal_value.to_f64(), 0.0); // Signal with no data should be zero
        assert_eq!(result.histogram_value.to_f64(), 0.0); // Histogram with no data should be zero
    }

    #[test]
    fn test_macd_single_input() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let result = macd.next(IndicatorValue::from(23.82));
        assert_eq!(result.macd_value.to_f64(), 0.0); // MACD with single input should be zero
        assert_eq!(result.signal_value.to_f64(), 0.0); // Signal with single input should be zero
        assert_eq!(result.histogram_value.to_f64(), 0.0); // Histogram with single input should be zero
    }

    #[test]
    fn test_macd_with_constant_prices() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices = vec![IndicatorValue::from(50.0); 26]; // Constant prices

        let result = macd.next_chunk(&prices);
        assert_eq!(result.macd_value.to_f64(), 0.0); // MACD should be zero with constant prices
        assert_eq!(result.signal_value.to_f64(), 0.0); // Signal should be zero with constant prices
        assert_eq!(result.histogram_value.to_f64(), 0.0); // Histogram should be zero with constant prices
    }

    #[test]
    fn test_macd_with_increasing_prices() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices: Vec<IndicatorValue> = (1..=26).map(|x| IndicatorValue::from(x as f64)).collect();

        let result = macd.next_chunk(&prices);
        assert!(result.macd_value.to_f64() > 0.0); // MACD should follow the increasing prices
        assert!(result.signal_value.to_f64() > 0.0); // Signal should follow the increasing prices
        assert!(result.histogram_value.to_f64() > 0.0); // Histogram should be positive
    }

    #[test]
    fn test_macd_with_decreasing_prices() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices: Vec<IndicatorValue> = (1..=26).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let result = macd.next_chunk(&prices);
        assert!(result.macd_value.to_f64() < 0.0); // MACD should follow the decreasing prices
        assert!(result.signal_value.to_f64() < 0.0); // Signal should follow the decreasing prices
        assert!(result.histogram_value.to_f64() < 0.0); // Histogram should be negative
    }

    #[test]
    fn test_macd_reset() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            macd.next(price);
        }

        macd.reset();
        let result = macd.next(IndicatorValue::from(23.82));
        assert_eq!(result.macd_value.to_f64(), 0.0); // After reset, it should start fresh
        assert_eq!(result.signal_value.to_f64(), 0.0);
        assert_eq!(result.histogram_value.to_f64(), 0.0);
    }
}
