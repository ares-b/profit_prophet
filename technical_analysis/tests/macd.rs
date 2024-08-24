#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{
        Indicator, MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceOutput,
    };
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_macd_next() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08,
            45.89, 46.03, 45.61, 46.28, 46.28, 46.0, 45.75, 46.15, 46.35, 46.55,
            46.75, 46.95, 47.15, 47.35, 47.55, 47.75, 47.95, 48.15, 48.35, 48.55,
            48.75, 48.95, 49.15, 49.35, 49.55, 49.75, 49.95, 50.15, 50.35, 50.55
        ]
        .into_iter()
        .map(IndicatorValue::from)
        .collect::<Vec<_>>();

        let mut result = None;
        for price in &prices {
            result = macd.next(*price);
        }
        let final_result = result.unwrap();
        assert_eq!(final_result.macd_value.round_dp(2), IndicatorValue::from(1.12));
        assert_eq!(final_result.signal_value.round_dp(2), IndicatorValue::from(1.02));
        assert_eq!(final_result.histogram_value.round_dp(2), IndicatorValue::from(0.10));
    }

    #[test]
    fn test_macd_with_increasing_prices() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices: Vec<IndicatorValue> = (1..=40).map(|x| IndicatorValue::from(x as f64)).collect();

        let expected_macd = IndicatorValue::from(6.39);
        let expected_signal = IndicatorValue::from(6.14);
        let expected_histogram = IndicatorValue::from(0.24);

        let mut result = None;
        for price in &prices {
            result = macd.next(*price);
        }

        if let Some(MovingAverageConvergenceDivergenceOutput {
            macd_value,
            signal_value,
            histogram_value,
        }) = result
        {
            assert_eq!(macd_value.round_dp(2), expected_macd);
            assert_eq!(signal_value.round_dp(2), expected_signal);
            assert_eq!(histogram_value.round_dp(2), expected_histogram);
        }
    }

    #[test]
    fn test_macd_with_decreasing_prices() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices: Vec<IndicatorValue> = (1..=40).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let expected_macd = IndicatorValue::from(-6.39);
        let expected_signal = IndicatorValue::from(-6.14);
        let expected_histogram = IndicatorValue::from(-0.24);

        let mut result = None;
        for price in &prices {
            result = macd.next(*price);
        }

        if let Some(MovingAverageConvergenceDivergenceOutput {
            macd_value,
            signal_value,
            histogram_value,
        }) = result
        {
            assert_eq!(macd_value.round_dp(2), expected_macd);
            assert_eq!(signal_value.round_dp(2), expected_signal);
            assert_eq!(histogram_value.round_dp(2), expected_histogram);
        }
    }

    #[test]
    fn test_macd_with_constant_prices() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices = vec![IndicatorValue::from(50.0); 40];

        let expected_macd = IndicatorValue::from(0.0);
        let expected_signal = IndicatorValue::from(0.0);
        let expected_histogram = IndicatorValue::from(0.0);

        let mut result = None;
        for price in &prices {
            result = macd.next(*price);
        }

        if let Some(MovingAverageConvergenceDivergenceOutput {
            macd_value,
            signal_value,
            histogram_value,
        }) = result
        {
            assert_eq!(macd_value.round_dp(2), expected_macd);
            assert_eq!(signal_value.round_dp(2), expected_signal);
            assert_eq!(histogram_value.round_dp(2), expected_histogram);
        }
    }

    #[test]
    fn test_macd_reset() {
        let mut macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            macd.next(price);
        }

        macd.reset();
        let result = macd.next(IndicatorValue::from(46.28));
        assert_eq!(result, None);
    }
}
