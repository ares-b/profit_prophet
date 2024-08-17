#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, IchimokuClouds};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_ichimoku_next() {
        let mut ichimoku = IchimokuClouds::default();
        let data = vec![
            (22.27, 21.19, 21.72), (22.29, 21.21, 21.75), (22.31, 21.23, 21.78), 
            (22.33, 21.25, 21.80), (22.35, 21.27, 21.83), (22.37, 21.29, 21.86),
            (22.39, 21.31, 21.88), (22.41, 21.33, 21.91), (22.43, 21.35, 21.94),
            (22.45, 21.37, 21.97), (22.47, 21.39, 21.99), (22.49, 21.41, 22.02),
            (22.51, 21.43, 22.05), (22.53, 21.45, 22.08), (22.55, 21.47, 22.10),
            (22.57, 21.49, 22.13), (22.59, 21.51, 22.16), (22.61, 21.53, 22.19),
            (22.63, 21.55, 22.21), (22.65, 21.57, 22.24),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            ichimoku.next(value);
        }

        let result = ichimoku.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59), IndicatorValue::from(22.26)));
        assert!(result.tenkan_sen.to_f64() > 0.0); // Tenkan Sen should be calculated
        assert!(result.kijun_sen.to_f64() > 0.0); // Kijun Sen should be calculated
        assert!(result.senkou_span_a.to_f64() > 0.0); // Senkou Span A should be calculated
        assert!(result.senkou_span_b.to_f64() > 0.0); // Senkou Span B should be calculated
        assert!(result.chikou_span.to_f64() > 0.0); // Chikou Span should be calculated
    }

    #[test]
    fn test_ichimoku_empty_input() {
        let mut ichimoku = IchimokuClouds::default();
        let result = ichimoku.next((IndicatorValue::from(0.0), IndicatorValue::from(0.0), IndicatorValue::from(0.0)));
        assert_eq!(result.tenkan_sen.to_f64(), 0.0); // Tenkan Sen with no data should be zero
        assert_eq!(result.kijun_sen.to_f64(), 0.0); // Kijun Sen with no data should be zero
        assert_eq!(result.senkou_span_a.to_f64(), 0.0); // Senkou Span A with no data should be zero
        assert_eq!(result.senkou_span_b.to_f64(), 0.0); // Senkou Span B with no data should be zero
        assert_eq!(result.chikou_span.to_f64(), 0.0); // Chikou Span with no data should be zero
    }

    #[test]
    fn test_ichimoku_single_input() {
        let mut ichimoku = IchimokuClouds::default();
        let result = ichimoku.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59), IndicatorValue::from(22.26)));
        assert_eq!(result.tenkan_sen.to_f64(), 22.13); // Tenkan Sen is the average of high and low
        assert_eq!(result.kijun_sen.to_f64(), 22.13); // Kijun Sen is the same when only one input
        assert_eq!(result.senkou_span_a.to_f64(), 22.13); // Span A should match Tenkan Sen with one input
        assert_eq!(result.senkou_span_b.to_f64(), 22.13); // Span B should match Kijun Sen with one input
        assert_eq!(result.chikou_span.to_f64(), 22.26); // Chikou Span is just the close value
    }

    #[test]
    fn test_ichimoku_with_constant_prices() {
        let mut ichimoku = IchimokuClouds::default();
        let data = vec![(22.55, 21.55, 22.10); 26]
            .into_iter()
            .map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c)))
            .collect::<Vec<_>>();

        let result = ichimoku.next_chunk(&data);
        assert_eq!(result.tenkan_sen.to_f64(), 22.05); // Tenkan Sen with constant prices should equal the average
        assert_eq!(result.kijun_sen.to_f64(), 22.05); // Kijun Sen with constant prices should equal the average
        assert_eq!(result.senkou_span_a.to_f64(), 22.05); // Span A should match average
        assert_eq!(result.senkou_span_b.to_f64(), 22.05); // Span B should match average
        assert_eq!(result.chikou_span.to_f64(), 22.10); // Chikou Span is just the last close value
    }

    #[test]
    fn test_ichimoku_reset() {
        let mut ichimoku = IchimokuClouds::default();
        let data = vec![
            (22.27, 21.19, 21.72), (22.29, 21.21, 21.75), (22.31, 21.23, 21.78), 
            (22.33, 21.25, 21.80),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        for value in data {
            ichimoku.next(value);
        }

        ichimoku.reset();
        let result = ichimoku.next((IndicatorValue::from(22.67), IndicatorValue::from(21.59), IndicatorValue::from(22.26)));
        assert_eq!(result.tenkan_sen.to_f64(), 22.13); // After reset, it should start fresh
        assert_eq!(result.kijun_sen.to_f64(), 22.13);
        assert_eq!(result.senkou_span_a.to_f64(), 22.13);
        assert_eq!(result.senkou_span_b.to_f64(), 22.13);
        assert_eq!(result.chikou_span.to_f64(), 22.26);
    }

    #[test]
    fn test_ichimoku_with_increasing_prices() {
        let mut ichimoku = IchimokuClouds::default();
        let data = (1..=52).map(|x| (IndicatorValue::from(x as f64), IndicatorValue::from((x - 1) as f64), IndicatorValue::from((x as f64 + 0.5) as f64)))
            .collect::<Vec<_>>();

        let result = ichimoku.next_chunk(&data);
        assert!(result.tenkan_sen.to_f64() > 0.0); // Increasing prices should give a positive Tenkan Sen
        assert!(result.kijun_sen.to_f64() > 0.0); // Increasing prices should give a positive Kijun Sen
        assert!(result.senkou_span_a.to_f64() > 0.0); // Increasing prices should give a positive Senkou Span A
        assert!(result.senkou_span_b.to_f64() > 0.0); // Increasing prices should give a positive Senkou Span B
        assert!(result.chikou_span.to_f64() > 0.0); // Increasing prices should give a positive Chikou Span
    }

    #[test]
    fn test_ichimoku_with_decreasing_prices() {
        let mut ichimoku = IchimokuClouds::default();
        let data = (1..=52).rev().map(|x| (IndicatorValue::from(x as f64), IndicatorValue::from((x - 1) as f64), IndicatorValue::from((x as f64 - 0.5) as f64)))
            .collect::<Vec<_>>();

        let result = ichimoku.next_chunk(&data);
        assert!(result.tenkan_sen.to_f64() > 0.0); // Decreasing prices should still give a positive Tenkan Sen, Kijun Sen, etc.
        assert!(result.kijun_sen.to_f64() > 0.0);
        assert!(result.senkou_span_a.to_f64() > 0.0);
        assert!(result.senkou_span_b.to_f64() > 0.0);
        assert!(result.chikou_span.to_f64() > 0.0);
    }

    #[test]
    fn test_ichimoku_with_extreme_values() {
        let mut ichimoku = IchimokuClouds::default();
        let data = vec![
            (10000.0, 1.0, 5000.0), (-10000.0, -5000.0, -7500.0),
        ].into_iter().map(|(h, l, c)| (IndicatorValue::from(h), IndicatorValue::from(l), IndicatorValue::from(c))).collect::<Vec<_>>();

        let result = ichimoku.next_chunk(&data);
        assert!(result.tenkan_sen.to_f64().abs() > 0.0); // Extreme values should still produce valid output
        assert!(result.kijun_sen.to_f64().abs() > 0.0);
        assert!(result.senkou_span_a.to_f64().abs() > 0.0);
        assert!(result.senkou_span_b.to_f64().abs() > 0.0);
        assert!(result.chikou_span.to_f64().abs() > 0.0);
    }
}
