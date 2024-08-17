#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, ChandeMomentumOscillator};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_cmo_next() {
        let mut cmo = ChandeMomentumOscillator::new(14);
        let data = vec![1.0, 2.0, 3.0, 2.5, 3.5, 2.7, 3.8, 2.6, 3.7, 2.9, 3.6, 2.8, 3.9, 2.7]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for value in data {
            cmo.next(value);
        }

        let result = cmo.next(IndicatorValue::from(3.0));
        assert!(result.to_f64() > 0.0); // CMO should be calculated
    }

    #[test]
    fn test_cmo_empty_input() {
        let mut cmo = ChandeMomentumOscillator::new(14);
        let result = cmo.next(IndicatorValue::from(0.0));
        assert_eq!(result.to_f64(), 0.0); // CMO with no data should be zero
    }

    #[test]
    fn test_cmo_single_input() {
        let mut cmo = ChandeMomentumOscillator::new(14);
        let result = cmo.next(IndicatorValue::from(3.0));
        assert_eq!(result.to_f64(), 100.0); // CMO with single positive input should be max positive
    }

    #[test]
    fn test_cmo_with_constant_prices() {
        let mut cmo = ChandeMomentumOscillator::new(14);
        let data = vec![IndicatorValue::from(50.0); 14]; // Constant prices

        let result = cmo.next_chunk(&data);
        assert_eq!(result.to_f64(), 0.0); // CMO should be zero with constant prices
    }

    #[test]
    fn test_cmo_with_increasing_prices() {
        let mut cmo = ChandeMomentumOscillator::new(14);
        let prices: Vec<IndicatorValue> = (1..=14).map(|x| IndicatorValue::from(x as f64)).collect();

        let result = cmo.next_chunk(&prices);
        assert!(result.to_f64() > 0.0); // CMO should be positive with increasing prices
    }

    #[test]
    fn test_cmo_with_decreasing_prices() {
        let mut cmo = ChandeMomentumOscillator::new(14);
        let prices: Vec<IndicatorValue> = (1..=14).rev().map(|x| IndicatorValue::from(x as f64)).collect();

        let result = cmo.next_chunk(&prices);
        assert!(result.to_f64() < 0.0); // CMO should be negative with decreasing prices
    }

    #[test]
    fn test_cmo_reset() {
        let mut cmo = ChandeMomentumOscillator::new(14);
        let prices = vec![1.0, 2.0, 3.0, 2.5, 3.5, 2.7, 3.8, 2.6, 3.7, 2.9, 3.6, 2.8, 3.9, 2.7]
            .into_iter()
            .map(IndicatorValue::from)
            .collect::<Vec<_>>();

        for price in prices {
            cmo.next(price);
        }

        cmo.reset();
        let result = cmo.next(IndicatorValue::from(3.0));
        assert_eq!(result.to_f64(), 100.0); // After reset, it should start fresh
    }
}
