#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, PercentagePriceOscillator};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_ppo_next() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);
        let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            ppo.next(price);
        }

        let result = ppo.next(IndicatorValue::from(23.82));
        assert!(result.ppo_value.to_f64() > 0.0); // PPO should be calculated
        assert!(result.signal_value.to_f64() > 0.0); // Signal should be calculated
        assert!(result.histogram_value.to_f64() > 0.0); // Histogram should be calculated
    }

    #[test]
    fn test_ppo_empty_input() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);
        let result = ppo.next(IndicatorValue::from(0.0));
        assert_eq!(result.ppo_value.to_f64(), 0.0); // PPO with no data should be zero
        assert_eq!(result.signal_value.to_f64(), 0.0); // Signal with no data should be zero
        assert_eq!(result.histogram_value.to_f64(), 0.0); // Histogram with no data should be zero
    }

    #[test]
    fn test_ppo_single_input() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);
        let result = ppo.next(IndicatorValue::from(23.82));
        assert_eq!(result.ppo_value.to_f64(), 0.0); // PPO with single input should be zero
        assert_eq!(result.signal_value.to_f64(), 0.0); // Signal with single input should be zero
        assert_eq!(result.histogram_value.to_f64(), 0.0); // Histogram with single input should be zero
    }

    #[test]
    fn test_ppo_with_constant_prices() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);
        let prices = vec![IndicatorValue::from(50.0); 26]; // Constant prices

        let result = ppo.next_chunk(&prices);
        assert_eq!(result.ppo_value.to_f64(), 0.0); // PPO should be zero with constant prices
        assert_eq!(result.signal_value.to_f64(), 0.0); // Signal should be zero with constant prices
        assert_eq!(result.histogram_value.to_f64(), 0.0); // Histogram should be zero with constant prices
    }

    #[test]
    fn test_ppo_with_increasing_prices() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);
        let prices: Vec<IndicatorValue> = (1..=26).map(|x| IndicatorValue::from(x as f64)).collect();

        let result = ppo.next_chunk(&prices);
        assert!(result.ppo_value.to_f64() > 0.0); // PPO should follow the increasing prices
        assert!(result.signal_value.to_f64() > 0.0); // Signal should follow the increasing prices
        assert!(result.histogram_value.to_f64() > 0.0); // Histogram should be positive
    }

    #[test]
    fn test_ppo_with_decreasing_prices() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);
        let prices: Vec<IndicatorValue> = (1..=26).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let result = ppo.next_chunk(&prices);
        assert!(result.ppo_value.to_f64() < 0.0); // PPO should follow the decreasing prices
        assert!(result.signal_value.to_f64() < 0.0); // Signal should follow the decreasing prices
        assert!(result.histogram_value.to_f64() < 0.0); // Histogram should be negative
    }

    #[test]
    fn test_ppo_reset() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);
        let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            ppo.next(price);
        }

        ppo.reset();
        let result = ppo.next(IndicatorValue::from(23.82));
        assert_eq!(result.ppo_value.to_f64(), 0.0); // After reset, it should start fresh
        assert_eq!(result.signal_value.to_f64(), 0.0);
        assert_eq!(result.histogram_value.to_f64(), 0.0);
    }
}
