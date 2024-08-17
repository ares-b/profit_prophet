#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, ParabolicSAR};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_parabolic_sar_next() {
        let mut sar = ParabolicSAR::new(0.02, 0.2);
        let data = vec![
            (22.27, 21.19), (22.29, 21.21), (22.31, 21.23), (22.33, 21.25),
            (22.35, 21.27), (22.37, 21.29), (22.39, 21.31), (22.41, 21.33),
            (22.43, 21.35), (22.45, 21.37), (22.47, 21.39), (22.49, 21.41),
            (22.51, 21.43), (22.53, 21.45), (22.55, 21.47), (22.57, 21.49),
            (22.59, 21.51), (22.61, 21.53), (22.63, 21.55), (22.65, 21.57),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        for value in data {
            sar.next(value);
        }

        let result = sar.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59)));
        assert!(result.to_f64() > 0.0); // SAR should be calculated
    }

    #[test]
    fn test_parabolic_sar_empty_input() {
        let mut sar = ParabolicSAR::new(0.02, 0.2);
        let result = sar.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result.to_f64(), 0.0); // SAR with no data should be zero
    }

    #[test]
    fn test_parabolic_sar_single_input() {
        let mut sar = ParabolicSAR::new(0.02, 0.2);
        let result = sar.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59)));
        assert_eq!(result.to_f64(), 22.67); // SAR with single input should equal the input
    }

    #[test]
    fn test_parabolic_sar_with_constant_prices() {
        let mut sar = ParabolicSAR::new(0.02, 0.2);
        let data = vec![(22.55, 21.55); 20]
            .into_iter()
            .map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l)))
            .collect::<Vec<_>>();

        let result = sar.next_chunk(&data);
        assert_eq!(result.to_f64(), 22.55); // SAR with constant prices should equal the high
    }

    #[test]
    fn test_parabolic_sar_with_increasing_prices() {
        let mut sar = ParabolicSAR::new(0.02, 0.2);
        let data: Vec<(IndicatorValue, IndicatorValue)> = (1..=20)
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64)))
            .collect();

        let result = sar.next_chunk(&data);
        assert!(result.to_f64() > 20.0); // SAR should follow the increasing prices
    }

    #[test]
    fn test_parabolic_sar_with_decreasing_prices() {
        let mut sar = ParabolicSAR::new(0.02, 0.2);
        let data: Vec<(IndicatorValue, IndicatorValue)> = (1..=20).rev()
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64)))
            .collect();

        let result = sar.next_chunk(&data);
        assert!(result.to_f64() < 20.0); // SAR should follow the decreasing prices
    }

    #[test]
    fn test_parabolic_sar_reset() {
        let mut sar = ParabolicSAR::new(0.02, 0.2);
        let data = vec![
            (22.27, 21.19), (22.29, 21.21), (22.31, 21.23), (22.33, 21.25),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        for value in data {
            sar.next(value);
        }

        sar.reset();
        let result = sar.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59)));
        assert_eq!(result.to_f64(), 22.67); // After reset, it should start fresh
    }
}
