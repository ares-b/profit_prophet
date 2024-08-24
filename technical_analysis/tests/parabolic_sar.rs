#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, ParabolicSAR};
    use technical_analysis::IndicatorValue;

     #[test]
    fn test_initialization_period() {
        let mut psar = ParabolicSAR::new(0.02, 0.2, 5);
        let data = [
            (IndicatorValue::from(10.0), IndicatorValue::from(5.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(6.0)),
            (IndicatorValue::from(11.0), IndicatorValue::from(7.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(14.0), IndicatorValue::from(9.0)),
        ];

        for value in data {
            assert_eq!(psar.next(value), None);
        }

        assert_eq!(psar.next(data[4]).unwrap().round_dp(2), IndicatorValue::from(5.18));
    }

    #[test]
    fn test_rising_market() {
        let mut psar = ParabolicSAR::new(0.02, 0.2, 5);
        let data = [
            (IndicatorValue::from(10.0), IndicatorValue::from(5.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(6.0)),
            (IndicatorValue::from(11.0), IndicatorValue::from(7.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(14.0), IndicatorValue::from(9.0)),
            (IndicatorValue::from(15.0), IndicatorValue::from(10.0)),
            (IndicatorValue::from(16.0), IndicatorValue::from(11.0)),
        ];

        for i in 0..5 {
            psar.next(data[i]);
        }

        assert_eq!(psar.next(data[5]).unwrap().round_dp(2), IndicatorValue::from(5.18));

        assert_eq!(psar.next(data[6]).unwrap().round_dp(2), IndicatorValue::from(5.57));
    }

    #[test]
    fn test_falling_market() {
        let mut psar = ParabolicSAR::new(0.02, 0.2, 5);
        let data = [
            (IndicatorValue::from(10.0), IndicatorValue::from(5.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(6.0)),
            (IndicatorValue::from(11.0), IndicatorValue::from(7.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(14.0), IndicatorValue::from(9.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(7.0)),
        ];

        for i in 0..5 {
            psar.next(data[i]);
        }

        assert_eq!(psar.next(data[5]).unwrap().round_dp(2), IndicatorValue::from(5.18));

        assert_eq!(psar.next(data[6]).unwrap().round_dp(2), IndicatorValue::from(5.36));
    }

    #[test]
    fn test_reversal_rising_to_falling() {
        let mut psar = ParabolicSAR::new(0.02, 0.2, 5);
        let data = [
            (IndicatorValue::from(10.0), IndicatorValue::from(5.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(6.0)),
            (IndicatorValue::from(11.0), IndicatorValue::from(7.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(14.0), IndicatorValue::from(9.0)),
            (IndicatorValue::from(15.0), IndicatorValue::from(10.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(7.0)),
        ];

        for i in 0..5 {
            psar.next(data[i]);
        }

        assert_eq!(psar.next(data[5]).unwrap().round_dp(2), IndicatorValue::from(5.18));
        assert_eq!(psar.next(data[6]).unwrap().round_dp(2), IndicatorValue::from(5.57));
    }

    #[test]
    fn test_reversal_falling_to_rising() {
        let mut psar = ParabolicSAR::new(0.02, 0.2, 5);
        let data = [
            (IndicatorValue::from(10.0), IndicatorValue::from(5.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(6.0)),
            (IndicatorValue::from(11.0), IndicatorValue::from(7.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(14.0), IndicatorValue::from(9.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(16.0), IndicatorValue::from(11.0)),
        ];

        for i in 0..5 {
            psar.next(data[i]);
        }

        assert_eq!(psar.next(data[5]).unwrap().round_dp(2), IndicatorValue::from(5.18));
        assert_eq!(psar.next(data[6]).unwrap().round_dp(2), IndicatorValue::from(5.36));
    }

    #[test]
    fn test_reset_functionality() {
        let mut psar = ParabolicSAR::new(0.02, 0.2, 5);
        let data = [
            (IndicatorValue::from(10.0), IndicatorValue::from(5.0)),
            (IndicatorValue::from(12.0), IndicatorValue::from(6.0)),
            (IndicatorValue::from(11.0), IndicatorValue::from(7.0)),
            (IndicatorValue::from(13.0), IndicatorValue::from(8.0)),
            (IndicatorValue::from(14.0), IndicatorValue::from(9.0)),
        ];

        for i in 0..5 {
            psar.next(data[i]);
        }

        psar.reset();

        assert_eq!(psar.next((IndicatorValue::from(14.0), IndicatorValue::from(9.0))), None);
    }
}
