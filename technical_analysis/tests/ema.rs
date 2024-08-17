#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, ExponentialMovingAverage};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_ema_next() {
        let mut ema = ExponentialMovingAverage::new(10);
        let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            ema.next(price);
        }

        let result = ema.next(IndicatorValue::from(23.82));
        assert_eq!(result.round_dp(2), IndicatorValue::from(22.54));
    }

    #[test]
    fn test_ema_empty_input() {
        let mut ema = ExponentialMovingAverage::new(10);
        let result = ema.next(IndicatorValue::from(0.0));
        assert_eq!(result.round_dp(2), IndicatorValue::from(0.0));
    }

    #[test]
    fn test_ema_single_input() {
        let mut ema = ExponentialMovingAverage::new(3);
        let result = ema.next(IndicatorValue::from(23.82));
        assert_eq!(result.round_dp(2), IndicatorValue::from(23.82));
    }

    #[test]
    fn test_ema_with_constant_prices() {
        let mut ema = ExponentialMovingAverage::new(10);
        let prices = vec![IndicatorValue::from(50.0); 10];

        let result = ema.next_chunk(&prices);
        assert_eq!(result.round_dp(2), IndicatorValue::from(50.0));
    }

    #[test]
    fn test_ema_with_increasing_prices() {
        let mut ema = ExponentialMovingAverage::new(10);
        let prices: Vec<IndicatorValue> = (1..=10).map(|x| IndicatorValue::from(x as f64)).collect();

        let result = ema.next_chunk(&prices);
        assert_eq!(result.round_dp(2), IndicatorValue::from(6.24));
    }

    #[test]
    fn test_ema_with_decreasing_prices() {
        let mut ema = ExponentialMovingAverage::new(10);
        let prices: Vec<IndicatorValue> = (1..=10).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let result = ema.next_chunk(&prices);
        assert_eq!(result.round_dp(2), IndicatorValue::from(4.76));
    }

    #[test]
    fn test_ema_reset() {
        let mut ema = ExponentialMovingAverage::new(10);
        let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            ema.next(price);
        }

        ema.reset();
        let result = ema.next(IndicatorValue::from(23.82));
        assert_eq!(result.round_dp(2), IndicatorValue::from(23.82));
    }
}
