#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Aroon, Indicator};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_aroon_single_input() {
        let mut aroon = Aroon::new(14);
        let result = aroon.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result, None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_aroon_next() {
        let mut aroon = Aroon::new(14);
        let data = vec![
            (22.27, 22.19), (22.29, 22.21), (22.31, 22.23), (22.33, 22.25),
            (22.35, 22.27), (22.37, 22.29), (22.39, 22.31), (22.41, 22.33),
            (22.43, 22.35), (22.45, 22.37), (22.47, 22.39), (22.49, 22.41),
            (22.51, 22.43), (22.53, 22.45),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        for value in data {
            aroon.next(value);
        }

        let result = aroon.next((IndicatorValue::from(22.55), IndicatorValue::from(22.47))).unwrap();
        assert_eq!(result.aroon_up.round_dp(2), IndicatorValue::from(100.0));
        assert_eq!(result.aroon_down.round_dp(2), IndicatorValue::from(7.14));
    }
    
    #[test]
    fn test_aroon_highest_lowest_at_end() {
        let mut aroon = Aroon::new(14);
        let data = vec![
            (22.27, 22.19), (22.29, 22.21), (22.31, 22.23), (22.33, 22.25),
            (22.35, 22.27), (22.37, 22.29), (22.39, 22.31), (22.41, 22.33),
            (22.43, 22.35), (22.45, 22.37), (22.47, 22.39), (22.49, 22.41),
            (22.51, 22.43), (22.53, 22.45),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        let result = aroon.next_chunk(&data).unwrap();
        assert_eq!(result.aroon_up.round_dp(2), IndicatorValue::from(100.0));
        assert_eq!(result.aroon_down.round_dp(2), IndicatorValue::from(7.14));
    }

    #[test]
    fn test_aroon_with_increasing_prices() {
        let mut aroon = Aroon::new(14);
        let data: Vec<(IndicatorValue, IndicatorValue)> = (1..=14).map(|x| (IndicatorValue::from(x as f64), IndicatorValue::from(x as f64 - 0.5))).collect();
        
        let result = aroon.next_chunk(&data).unwrap();
        assert_eq!(result.aroon_up.round_dp(2), IndicatorValue::from(100.0));
        assert_eq!(result.aroon_down.round_dp(2), IndicatorValue::from(7.14));
    }

    #[test]
    fn test_aroon_with_decreasing_prices() {
        let mut aroon = Aroon::new(14);
        let data = vec![
            (IndicatorValue::from(14.0), IndicatorValue::from(13.5)),
            (IndicatorValue::from(3.0), IndicatorValue::from(2.5)),
            (IndicatorValue::from(12.0), IndicatorValue::from(11.5)),
            (IndicatorValue::from(1.0), IndicatorValue::from(0.5)),
            (IndicatorValue::from(10.0), IndicatorValue::from(9.5)),
            (IndicatorValue::from(9.0), IndicatorValue::from(8.5)),
            (IndicatorValue::from(8.0), IndicatorValue::from(7.5)),
            (IndicatorValue::from(7.0), IndicatorValue::from(6.5)),
            (IndicatorValue::from(6.0), IndicatorValue::from(5.5)),
            (IndicatorValue::from(5.0), IndicatorValue::from(4.5)),
            (IndicatorValue::from(4.0), IndicatorValue::from(3.5)), 
            (IndicatorValue::from(3.0), IndicatorValue::from(2.5)),
            (IndicatorValue::from(2.0), IndicatorValue::from(1.5)),
            (IndicatorValue::from(1.0), IndicatorValue::from(0.5))
        ];
        
        let result = aroon.next_chunk(&data).unwrap();
        
        assert_eq!(result.aroon_up.round_dp(2), IndicatorValue::from(7.14));
        assert_eq!(result.aroon_down.round_dp(2), IndicatorValue::from(100.0));
    }

    #[test]
    fn test_aroon_reset() {
        let mut aroon = Aroon::new(14);
        let data = vec![
            (22.27, 22.19), (22.29, 22.21), (22.31, 22.23), (22.33, 22.25),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        for value in data {
            aroon.next(value);
        }

        aroon.reset();
        let result = aroon.next((IndicatorValue::from(22.55), IndicatorValue::from(22.47)));
        assert_eq!(result, None);
        assert_eq!(result, None);
    }
}
