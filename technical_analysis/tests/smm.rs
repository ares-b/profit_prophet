#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, SimpleMovingMedian};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_basic_functionality() {
        let mut smm = SimpleMovingMedian::new(3);

        assert_eq!(smm.next(IndicatorValue::from(10.0)), IndicatorValue::from(10.0));
        assert_eq!(smm.next(IndicatorValue::from(20.0)), IndicatorValue::from(15.0));
        assert_eq!(smm.next(IndicatorValue::from(30.0)), IndicatorValue::from(20.0));
    }

    #[test]
    fn test_empty_buffer() {
        let mut smm = SimpleMovingMedian::new(3);
        assert_eq!(smm.next(IndicatorValue::from(0.0)), 0.0.into());
    }

    #[test]
    fn test_single_element() {
        let mut smm = SimpleMovingMedian::new(3);
        assert_eq!(smm.next(IndicatorValue::from(25.0)), IndicatorValue::from(25.0));
    }

    #[test]
    fn test_full_buffer() {
        let mut smm = SimpleMovingMedian::new(3);
        assert_eq!(smm.next(IndicatorValue::from(10.0)), IndicatorValue::from(10.0));
        assert_eq!(smm.next(IndicatorValue::from(10.0)), IndicatorValue::from(10.0));
        assert_eq!(smm.next(IndicatorValue::from(20.0)), IndicatorValue::from(10.0));
        assert_eq!(smm.next(IndicatorValue::from(30.0)), IndicatorValue::from(20.0));
        assert_eq!(smm.next(IndicatorValue::from(40.0)), IndicatorValue::from(30.0));
        assert_eq!(smm.next(IndicatorValue::from(50.0)), IndicatorValue::from(40.0));
    }

    #[test]
    fn test_reset_functionality() {
        let mut smm = SimpleMovingMedian::new(3);
        smm.next(IndicatorValue::from(10.0));
        smm.next(IndicatorValue::from(20.0));
        smm.next(IndicatorValue::from(30.0));

        smm.reset();

        assert_eq!(smm.next(IndicatorValue::from(0.0)), 0.0.into());
        assert_eq!(smm.next(IndicatorValue::from(40.0)), IndicatorValue::from(20.0));
    }

    #[test]
    fn test_next_chunk() {
        let mut smm = SimpleMovingMedian::new(3);

        // Pass a chunk of values
        assert_eq!(smm.next_chunk(&[
            IndicatorValue::from(10.0),
            IndicatorValue::from(20.0),
            IndicatorValue::from(30.0),
        ]), IndicatorValue::from(20.0)); // Median of [10, 20, 30] should be 20
        assert_eq!(smm.next_chunk(&[
            IndicatorValue::from(40.0),
            IndicatorValue::from(50.0),
        ]), IndicatorValue::from(40.0)); // Median of [30, 40, 50] should be 40
    }
}
