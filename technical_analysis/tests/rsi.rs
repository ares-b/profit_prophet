#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, RelativeStrengthIndex};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_rsi_next() {
        let mut rsi = RelativeStrengthIndex::new(14);
        assert_eq!((rsi.next(IndicatorValue::from(44.34)) * 100.00.into()).round(2) / 100.00.into(), 50.00.into());
        assert_eq!((rsi.next(IndicatorValue::from(44.09)) * 100.00.into()).round(2) / 100.00.into(), 0.0.into());
        assert_eq!((rsi.next(IndicatorValue::from(44.15)) * 100.00.into()).round(2) / 100.00.into(), 3.56.into());
        assert_eq!((rsi.next(IndicatorValue::from(43.61)) * 100.00.into()).round(2) / 100.00.into(), 2.6.into());
        assert_eq!((rsi.next(IndicatorValue::from(44.33)) * 100.00.into()).round(2) / 100.00.into(), 31.18.into());
        assert_eq!((rsi.next(IndicatorValue::from(44.83)) * 100.00.into()).round(2) / 100.00.into(), 44.28.into());
    }
    #[test]
    fn test_rsi_next_chunk() {
        let mut rsi = RelativeStrengthIndex::new(14);
        let prices = vec![
             44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 
             45.89, 46.03, 45.61, 46.28
        ].into_iter().map(IndicatorValue::from).collect::<Vec<_>>();

        let result = rsi.next_chunk(&prices);
        assert_eq!((result.to_f64() * 100.00).round(2) / 100.00, 64.35);
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
        assert_eq!(result.to_f64(), 50.0);
    }

    // #[test]
    // fn test_rsi_with_constant_prices() {
    //     let mut rsi = RelativeStrengthIndex::new(14);
    //     let prices = vec![IndicatorValue::from(50.0); 20]; // Constant prices

    //     let result = rsi.next_chunk(&prices);
    //     assert_eq!(result.to_f64(), 50.0); // RSI should be 50 when there's no gain or loss
    // }

    // #[test]
    // fn test_rsi_with_increasing_prices() {
    //     let mut rsi = RelativeStrengthIndex::new(14);
    //     let prices: Vec<IndicatorValue> = (1..=20).map(|x| IndicatorValue::from(x as f64)).collect();

    //     let result = rsi.next_chunk(&prices);
    //     assert!(result.to_f64() > 50.0); // RSI should be above 50 with increasing prices
    // }

    // #[test]
    // fn test_rsi_with_decreasing_prices() {
    //     let mut rsi = RelativeStrengthIndex::new(14);
    //     let prices: Vec<IndicatorValue> = (1..=20).rev().map(|x| IndicatorValue::from(x as f64)).collect();

    //     let result = rsi.next_chunk(&prices);
    //     assert!(result.to_f64() < 50.0); // RSI should be below 50 with decreasing prices
    // }
}
