#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, RelativeStrengthIndex};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_rsi_next() {
        let mut rsi = RelativeStrengthIndex::new(14);

        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 
            45.89, 46.03, 45.61, 46.28, 46.28, 46.0, 45.75, 46.15, 46.35, 46.55, 46.55, 46.55, 46.55
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for i in 0..prices.len() {
            
            if i < 14 {
                assert_eq!(rsi.next(prices[i]), None);
            } else {
                let expected_value = match i {
                    14 => IndicatorValue::from(70.46),
                    15 => IndicatorValue::from(66.25),
                    16 => IndicatorValue::from(62.65),
                    17 => IndicatorValue::from(65.85),
                    18 => IndicatorValue::from(67.35),
                    19 => IndicatorValue::from(68.83),
                    20 => IndicatorValue::from(68.83),
                    21 => IndicatorValue::from(68.83),
                    22 => IndicatorValue::from(68.83),
                    _ => 50.0.into(),
                };
                assert_eq!(rsi.next(prices[i]).unwrap().round_dp(2), expected_value);
            }
        }
    }

    #[test]
    fn test_rsi_next_chunk() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 
            45.89, 46.03, 45.61, 46.28
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        let result = rsi.next_chunk(&prices);
        assert_eq!(result, None);

        let more_prices = vec![
            46.28, 46.0, 45.75, 46.15, 46.35, 46.55
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        let result = rsi.next_chunk(&more_prices);
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(68.83));
    }

    #[test]
    fn test_rsi_reset() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 
            45.89, 46.03, 45.61, 46.28
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        for price in prices {
            rsi.next(price);
        }

        rsi.reset();
        let result = rsi.next(IndicatorValue::from(46.28));
        assert_eq!(result, None);
    }

    #[test]
    fn test_rsi_with_constant_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![IndicatorValue::from(50.0); 20];

        let mut result = None;

        for value in &prices {
            result = rsi.next(*value);
        }
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(50.0));
    }

    #[test]
    fn test_rsi_with_increasing_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices: Vec<IndicatorValue> = (1..=20).map(|x| IndicatorValue::from(x as f64)).collect();

        let mut result = None;
        for price in &prices {
            result = rsi.next(*price);
        }
        
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(100.0));
    }

    #[test]
    fn test_rsi_with_decreasing_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices: Vec<IndicatorValue> = (1..=20).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let mut result = None;
        for price in &prices {
            result = rsi.next(*price);
        }
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(0.0));
    }

    #[test]
    fn test_rsi_with_single_data_point() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let price = IndicatorValue::from(50.0);

        let result = rsi.next(price);
        assert_eq!(result, None);  // RSI should return None with only one data point
    }

    #[test]
    fn test_rsi_with_all_zero_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![IndicatorValue::from(0.0); 20];

        let mut result = None;

        for value in &prices {
            result = rsi.next(*value);
        }
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(50.0));
    }

    #[test]
    fn test_rsi_with_max_float_values() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![IndicatorValue::from(f64::MAX); 20];

        let mut result = None;

        for value in &prices {
            result = rsi.next(*value);
        }
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(50.0));
    }

    #[test]
    fn test_rsi_with_period_greater_than_data_points() {
        let mut rsi = RelativeStrengthIndex::new(30);
        let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            let result = rsi.next(price);
            assert_eq!(result, None);
        }
    }

    #[test]
    fn test_rsi_with_mixed_gains_and_losses() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![
            44.34, 43.09, 45.15, 43.61, 46.33, 44.83, 45.10, 45.42, 45.84, 44.08, 
            45.89, 46.03, 45.61, 47.28, 46.28, 46.0, 45.75, 46.15
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        let mut result = None;

        for value in &prices {
            result = rsi.next(*value);
        }

        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(55.14));
    }

    #[test]
    fn test_rsi_with_alternating_prices() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices: Vec<IndicatorValue> = (1..=20).map(|x| if x % 2 == 0 { IndicatorValue::from(50.0) } else { IndicatorValue::from(100.0) }).collect();
        let mut result = None;

        for value in &prices {
            result = rsi.next(*value);
        }

        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(46.87));
    }
}
