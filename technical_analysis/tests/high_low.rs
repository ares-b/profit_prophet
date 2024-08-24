#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, HighLow, High, Low};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_high_only() {
        let mut high_indicator = High::new(3);

        let output = high_indicator.next(IndicatorValue::from(10.0));
        assert_eq!(output, None);
        assert_eq!(output, None);
        
        let output = high_indicator.next(IndicatorValue::from(20.0));
        assert_eq!(output, None);
        assert_eq!(output, None);


        let output = high_indicator.next(IndicatorValue::from(5.0)).unwrap();
        assert_eq!(output.high_value, IndicatorValue::from(20.0));
        assert_eq!(output.high_index, 1);

        
        let output = high_indicator.next(IndicatorValue::from(30.0)).unwrap();

        assert_eq!(output.high_value, IndicatorValue::from(30.0));
        assert_eq!(output.high_index, 0);

        let output = high_indicator.next(IndicatorValue::from(15.0)).unwrap();
        assert_eq!(output.high_value, IndicatorValue::from(30.0));
        assert_eq!(output.high_index, 0);
    }

    #[test]
    fn test_low_only() {
        let mut low_indicator = Low::new(3);

        let output = low_indicator.next(IndicatorValue::from(10.0));
        assert_eq!(output, None);
        assert_eq!(output, None);
        
        let output = low_indicator.next(IndicatorValue::from(5.0));
        assert_eq!(output, None);
        assert_eq!(output, None);

        let output = low_indicator.next(IndicatorValue::from(15.0)).unwrap();
        assert_eq!(output.low_value, IndicatorValue::from(5.0));
        assert_eq!(output.low_index, 1);

        // Overflow, the oldest value (10.0) is pushed out
        let output = low_indicator.next(IndicatorValue::from(3.0)).unwrap();
        assert_eq!(output.low_value, IndicatorValue::from(3.0));
        assert_eq!(output.low_index, 0);

        let output = low_indicator.next(IndicatorValue::from(12.0)).unwrap();
        assert_eq!(output.low_value, IndicatorValue::from(3.0));
        assert_eq!(output.low_index, 0);
    }

    #[test]
    fn test_high_and_low() {
        let mut high_low_indicator = HighLow::new(3);

        let output = high_low_indicator.next(IndicatorValue::from(10.0));
        assert_eq!(output, None);
        assert_eq!(output, None);
        assert_eq!(output, None);
        assert_eq!(output, None);

        let output = high_low_indicator.next(IndicatorValue::from(20.0));
        assert_eq!(output, None);
        assert_eq!(output, None);
        assert_eq!(output, None);
        assert_eq!(output, None);

        let output = high_low_indicator.next(IndicatorValue::from(5.0)).unwrap();
        assert_eq!(output.high_value, IndicatorValue::from(20.0));
        assert_eq!(output.high_index, 1);
        assert_eq!(output.low_value, IndicatorValue::from(5.0));
        assert_eq!(output.low_index, 2);

        // Overflow, the oldest value (10.0) is pushed out
        let output = high_low_indicator.next(IndicatorValue::from(15.0)).unwrap();
        assert_eq!(output.high_value, IndicatorValue::from(20.0));
        assert_eq!(output.high_index, 1);
        assert_eq!(output.low_value, IndicatorValue::from(5.0));
        assert_eq!(output.low_index, 2);

        // Test edge case where the low is pushed out
        let output = high_low_indicator.next(IndicatorValue::from(25.0)).unwrap();
        assert_eq!(output.high_value, IndicatorValue::from(25.0));
        assert_eq!(output.high_index, 0);
        assert_eq!(output.low_value, IndicatorValue::from(5.0));
        assert_eq!(output.low_index, 2);
    }

    // #[test]
    // fn test_reset() {
    //     let mut high_low_indicator = HighLow::new(3);

    //     high_low_indicator.next(IndicatorValue::from(10.0));
    //     high_low_indicator.next(IndicatorValue::from(20.0));
    //     high_low_indicator.next(IndicatorValue::from(5.0));

    //     high_low_indicator.reset();
    //     let output = high_low_indicator.next(IndicatorValue::from(15.0));

    //     assert_eq!(output.high_value, IndicatorValue::from(15.0)));
    //     assert_eq!(output.high_index, 0));
    //     assert_eq!(output.low_value, IndicatorValue::from(15.0)));
    //     assert_eq!(output.low_index, 0));
    // }
}
