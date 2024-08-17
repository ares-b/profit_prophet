#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, ChaikinMoneyFlow};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_cmf_next() {
        let mut cmf = ChaikinMoneyFlow::new(20);
        let data = vec![
            (22.27, 22.19, 22.25, 2000.0), (22.29, 22.21, 22.28, 1500.0),
            (22.31, 22.23, 22.30, 1800.0), (22.33, 22.25, 22.32, 1700.0),
            (22.35, 22.27, 22.34, 1600.0), (22.37, 22.29, 22.36, 1900.0),
            (22.39, 22.31, 22.38, 2000.0), (22.41, 22.33, 22.40, 2100.0),
            (22.43, 22.35, 22.42, 2200.0), (22.45, 22.37, 22.44, 2300.0),
            (22.47, 22.39, 22.46, 2400.0), (22.49, 22.41, 22.48, 2500.0),
            (22.51, 22.43, 22.50, 2600.0), (22.53, 22.45, 22.52, 2700.0),
            (22.55, 22.47, 22.54, 2800.0), (22.57, 22.49, 22.56, 2900.0),
            (22.59, 22.51, 22.58, 3000.0), (22.61, 22.53, 22.60, 3100.0),
            (22.63, 22.55, 22.62, 3200.0), (22.65, 22.57, 22.64, 3300.0),
        ].into_iter().map(|(h, l, c, v)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c), IndicatorValue::from(v))).collect::<Vec<_>>();

        for value in data {
            cmf.next(value);
        }

        let result = cmf.next((IndicatorValue::from(22.67), IndicatorValue::from(22.59), IndicatorValue::from(22.66), IndicatorValue::from(3400.0)));
        assert!(result.to_f64() > 0.0); // CMF should be calculated
    }

    #[test]
    fn test_cmf_empty_input() {
        let mut cmf = ChaikinMoneyFlow::new(20);
        let result = cmf.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result.to_f64(), 0.0); // CMF with no data should be zero
    }

    #[test]
    fn test_cmf_single_input() {
        let mut cmf = ChaikinMoneyFlow::new(20);
        let result = cmf.next((IndicatorValue::from(22.67), IndicatorValue::from(22.59), IndicatorValue::from(22.66), IndicatorValue::from(3400.0)));
        assert!(result.to_f64() > 0.0); // CMF with single input should be non-zero
    }

    #[test]
    fn test_cmf_reset() {
        let mut cmf = ChaikinMoneyFlow::new(20);
        let data = vec![
            (22.27, 22.19, 22.25, 2000.0), (22.29, 22.21, 22.28, 1500.0),
            (22.31, 22.23, 22.30, 1800.0),
        ].into_iter().map(|(h, l, c, v)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c), IndicatorValue::from(v))).collect::<Vec<_>>();

        for value in data {
            cmf.next(value);
        }

        cmf.reset();
        let result = cmf.next((IndicatorValue::from(22.67), IndicatorValue::from(22.59), IndicatorValue::from(22.66), IndicatorValue::from(3400.0)));
        assert!(result.to_f64() > 0.0); // CMF should be recalculated after reset
    }

    #[test]
    fn test_cmf_with_constant_prices() {
        let mut cmf = ChaikinMoneyFlow::new(20);
        let data = vec![(22.55, 22.55, 22.55, 2000.0); 20]
            .into_iter()
            .map(|(h, l, c, v)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c), IndicatorValue::from(v)))
            .collect::<Vec<_>>();

        let result = cmf.next_chunk(&data);
        assert_eq!(result.to_f64(), 0.0); // CMF should be zero when prices are constant
    }

    #[test]
    fn test_cmf_with_increasing_prices() {
        let mut cmf = ChaikinMoneyFlow::new(20);
        let data: Vec<(IndicatorValue, IndicatorValue, IndicatorValue, IndicatorValue)> = (1..=20)
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64), IndicatorValue::from(x as f64 + 0.5), IndicatorValue::from(x as f64 * 100.0)))
            .collect();

        let result = cmf.next_chunk(&data);
        assert!(result.to_f64() > 0.0); // CMF should be calculated correctly with increasing prices
    }

    #[test]
    fn test_cmf_with_decreasing_prices() {
        let mut cmf = ChaikinMoneyFlow::new(20);
        let data: Vec<(IndicatorValue, IndicatorValue, IndicatorValue, IndicatorValue)> = (1..=20).rev()
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64), IndicatorValue::from(x as f64 + 0.5), IndicatorValue::from(x as f64 * 100.0)))
            .collect();

        let result = cmf.next_chunk(&data);
        assert!(result.to_f64() < 0.0); // CMF should be calculated correctly with decreasing prices
    }
}
