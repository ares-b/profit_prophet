#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, MeanAbsDev};

    #[test]
    fn test_mad_with_empty_buffer() {
        let mut mad = MeanAbsDev::new(5);
        assert_eq!(mad.next(0.0.into()), 0.0.into());
    }

    #[test]
    fn test_mad_with_partial_buffer() {
        let mut mad = MeanAbsDev::new(3);
        mad.next(1.0.into());
        mad.next(2.0.into());
        let result = (mad.next(3.0.into()) * 100.00.into()).round(2) / 100.00.into();
        assert_eq!(result, 0.67.into());
    }

    #[test]
    fn test_mad_with_full_buffer() {
        let mut mad = MeanAbsDev::new(3);
        mad.next(1.0.into());
        mad.next(2.0.into());
        mad.next(3.0.into());
        let result = (mad.next(4.0.into()) * 100.00.into()).round(2) / 100.00.into();
        assert_eq!(result, 0.67.into());
    }

    #[test]
    fn test_mad_reset() {
        let mut mad = MeanAbsDev::new(3);
        mad.next(1.0.into());
        mad.next(2.0.into());
        mad.reset();
        let result = (mad.next(3.0.into()) * 100.00.into()).round(2) / 100.00.into();
        assert_eq!(result, 0.0.into());
    }

    #[test]
    fn test_mad_large_input() {
        let mut mad = MeanAbsDev::new(5);
        mad.next(10.0.into());
        mad.next(20.0.into());
        mad.next(30.0.into());
        mad.next(40.0.into());
        mad.next(50.0.into());
        let result = (mad.next(60.0.into()) * 100.00.into()).round(2) / 100.00.into();
        assert_eq!(result, 12.0.into());
    }
}
