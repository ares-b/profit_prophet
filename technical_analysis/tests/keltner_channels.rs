#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, KeltnerChannels};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_keltner_channels_next() {
        let mut kc = KeltnerChannels::new(20, 2.0);
        let data = vec![
            (22.27, 21.19, 22.0), (22.29, 21.21, 22.1), (22.31, 21.23, 22.2),
            (22.33, 21.25, 22.3), (22.35, 21.27, 22.4), (22.37, 21.29, 22.5),
            (22.39, 21.31, 22.6), (22.41, 21.33, 22.7), (22.43, 21.35, 22.8),
            (22.45, 21.37, 22.9), (22.47, 21.39, 23.0), (22.49, 21.41, 23.1),
            (22.51, 21.43, 23.2), (22.53, 21.45, 23.3), (22.55, 21.47, 23.4),
            (22.57, 21.49, 23.5), (22.59, 21.51, 23.6), (22.61, 21.53, 23.7),
            (22.63, 21.55, 23.8), (22.65, 21.57, 23.9),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            kc.next(value);
        }

        let result = kc.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59), IndicatorValue::from(22.60)));
        assert!(result.upper_band.to_f64() > 0.0); // Upper band should be calculated
        assert!(result.lower_band.to_f64() > 0.0); // Lower band should be calculated
    }

    #[test]
    fn test_keltner_channels_empty_input() {
        let mut kc = KeltnerChannels::new(20, 2.0);
        let result = kc.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result.upper_band.to_f64(), 0.0); // Upper band with no data should be zero
        assert_eq!(result.lower_band.to_f64(), 0.0); // Lower band with no data should be zero
    }

    #[test]
    fn test_keltner_channels_single_input() {
        let mut kc = KeltnerChannels::new(20, 2.0);
        let result = kc.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59), IndicatorValue::from(22.60)));
        assert_eq!(result.upper_band.to_f64(), 22.60); // With one value, upper and lower should match the input
        assert_eq!(result.lower_band.to_f64(), 22.60);
    }

    #[test]
    fn test_keltner_channels_with_constant_prices() {
        let mut kc = KeltnerChannels::new(20, 2.0);
        let data = vec![(22.55, 21.55, 22.55); 20]
            .into_iter()
            .map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c)))
            .collect::<Vec<_>>();

        let result = kc.next_chunk(&data);
        assert_eq!(result.upper_band.to_f64(), 22.55); // Upper band with constant prices should equal closing price
        assert_eq!(result.lower_band.to_f64(), 22.55); // Lower band with constant prices should equal closing price
    }

    #[test]
    fn test_keltner_channels_with_increasing_prices() {
        let mut kc = KeltnerChannels::new(20, 2.0);
        let data: Vec<(IndicatorValue, IndicatorValue, IndicatorValue)> = (1..=20)
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64), IndicatorValue::from(x as f64 + 0.5)))
            .collect();

        let result = kc.next_chunk(&data);
        assert!(result.upper_band.to_f64() > 20.5); // Upper band should follow the increasing prices
        assert!(result.lower_band.to_f64() < 20.5); // Lower band should follow the increasing prices
    }

    #[test]
    fn test_keltner_channels_with_decreasing_prices() {
        let mut kc = KeltnerChannels::new(20, 2.0);
        let data: Vec<(IndicatorValue, IndicatorValue, IndicatorValue)> = (1..=20).rev()
            .map(|x| (IndicatorValue::from(x as f64 + 1.0), IndicatorValue::from(x as f64), IndicatorValue::from(x as f64 + 0.5)))
            .collect();

        let result = kc.next_chunk(&data);
        assert!(result.upper_band.to_f64() > result.lower_band.to_f64()); // Upper band should be above lower band
    }

    #[test]
    fn test_keltner_channels_reset() {
        let mut kc = KeltnerChannels::new(20, 2.0);
        let data = vec![
            (22.27, 21.19, 22.0), (22.29, 21.21, 22.1), (22.31, 21.23, 22.2),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            kc.next(value);
        }

        kc.reset();
        let result = kc.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59), IndicatorValue::from(22.60)));
        assert_eq!(result.upper_band.to_f64(), 22.60); // After reset, it should start fresh
        assert_eq!(result.lower_band.to_f64(), 22.60); 
    }
}
