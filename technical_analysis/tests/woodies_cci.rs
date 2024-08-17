#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, WoodiesCCI};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_woodies_cci_next() {
        let mut cci = WoodiesCCI::new(14);
        let data = vec![
            (22.27, 21.19, 22.25), (22.29, 21.21, 22.28), (22.31, 21.23, 22.30),
            (22.33, 21.25, 22.32), (22.35, 21.27, 22.34), (22.37, 21.29, 22.36),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            cci.next(value);
        }

        let result = cci.next((IndicatorValue::from(22.39), IndicatorValue::from(21.31), IndicatorValue::from(22.38)));
        assert!(result.to_f64() > 0.0); // CCI should be calculated
    }

    #[test]
    fn test_woodies_cci_empty_input() {
        let mut cci = WoodiesCCI::new(14);
        let result = cci.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result.to_f64(), 0.0); // CCI with no data should be zero
    }

    #[test]
    fn test_woodies_cci_single_input() {
        let mut cci = WoodiesCCI::new(14);
        let result = cci.next((IndicatorValue::from(22.39), IndicatorValue::from(21.31), IndicatorValue::from(22.38)));
        assert!(result.to_f64() > 0.0); // CCI with single input should be non-zero
    }

    #[test]
    fn test_woodies_cci_with_constant_prices() {
        let mut cci = WoodiesCCI::new(14);
        let data = vec![(22.55, 21.55, 22.55); 14]
            .into_iter()
            .map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c)))
            .collect::<Vec<_>>();

        let result = cci.next_chunk(&data);
        assert_eq!(result.to_f64(), 0.0); // CCI should be zero with constant prices
    }

    #[test]
    fn test_woodies_cci_reset() {
        let mut cci = WoodiesCCI::new(14);
        let data = vec![
            (22.27, 21.19, 22.25), (22.29, 21.21, 22.28),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            cci.next(value);
        }

        cci.reset();
        let result = cci.next((IndicatorValue::from(22.39), IndicatorValue::from(21.31), IndicatorValue::from(22.38)));
        assert!(result.to_f64() > 0.0); // After reset, it should start fresh
    }
}
