#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, AverageTrueRange};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_atr() {
        let mut atr = AverageTrueRange::new(10);
        assert_eq!(
            atr.next((IndicatorValue::from(46.78), IndicatorValue::from(46.34), IndicatorValue::from(46.43))),
            None
        );
        assert_eq!(
            atr.next((IndicatorValue::from(47.32), IndicatorValue::from(46.64), IndicatorValue::from(46.87))),
            None
        );
        assert_eq!(
            atr.next((IndicatorValue::from(47.45), IndicatorValue::from(46.43), IndicatorValue::from(46.76))),
            None
        );
        
    }

    #[test]
    fn test_atr_next() {
        let mut atr = AverageTrueRange::new(14);
        let data = vec![
            (22.27, 21.19, 21.50), (22.29, 21.21, 22.00), (22.31, 21.23, 21.80),
            (22.33, 21.25, 22.10), (22.35, 21.27, 22.30), (22.37, 21.29, 21.90),
            (22.39, 21.31, 22.20), (22.41, 21.33, 21.95), (22.43, 21.35, 22.50),
            (22.45, 21.37, 21.80), (22.47, 21.39, 22.40), (22.49, 21.41, 22.60),
            (22.51, 21.43, 22.70), (22.53, 21.45, 22.85),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            atr.next(value);
        }

        let result = atr.next((IndicatorValue::from(22.55), IndicatorValue::from(21.47), IndicatorValue::from(22.65))).unwrap();
        assert_eq!(result.round_dp(2), IndicatorValue::from(1.12));
    }

    #[test]
    fn test_atr_empty_input() {
        let mut atr = AverageTrueRange::new(14);
        let result = atr.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result, None);
    }

    #[test]
    fn test_atr_single_input() {
        let mut atr = AverageTrueRange::new(2);
        atr.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        let result = atr.next((IndicatorValue::from(22.55), IndicatorValue::from(21.47), IndicatorValue::from(22.65))).unwrap();
        assert_eq!(result.round_dp(2), IndicatorValue::from(11.28));
    }

    #[test]
    fn test_atr_reset() {
        let mut atr = AverageTrueRange::new(14);
        let data = vec![
            (22.27, 21.19, 21.50), (22.29, 21.21, 22.00), (22.31, 21.23, 21.80),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            atr.next(value);
        }

        atr.reset();
        let result = atr.next((IndicatorValue::from(22.55), IndicatorValue::from(21.47), IndicatorValue::from(22.65)));
        assert_eq!(result, None);
    }

    #[test]
    fn test_atr_with_constant_prices() {
        let mut atr = AverageTrueRange::new(14);
        let data = vec![(22.55, 22.55, 22.55); 14]
            .into_iter()
            .map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c)))
            .collect::<Vec<_>>();

        let result = atr.next_chunk(&data).unwrap();
        assert_eq!(result.round_dp(2), IndicatorValue::from(0.0));
    }

    #[test]
    fn test_atr_with_increasing_prices() {
        let mut atr = AverageTrueRange::new(14);
        let data: Vec<(IndicatorValue, IndicatorValue, IndicatorValue)> = (1..=14)
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64), IndicatorValue::from(x as f64 + 0.5)))
            .collect();

        let result = atr.next_chunk(&data).unwrap();
        assert_eq!(result.round_dp(2), IndicatorValue::from(1.46));
    }
    #[test]
    fn test_atr_with_decreasing_prices() {
        let mut atr = AverageTrueRange::new(14);
        let data: Vec<(IndicatorValue, IndicatorValue, IndicatorValue)> = (1..=14).rev()
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64), IndicatorValue::from(x as f64 + 0.5)))
            .collect();

        let result = atr.next_chunk(&data).unwrap();
        assert_eq!(result.round_dp(2), IndicatorValue::from(1.46));
    }
}
