#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, CommodityChannelIndex};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_cci_next() {
        let mut cci = CommodityChannelIndex::new(3);
        assert_eq!(cci.next((1.0.into(), 0.5.into(), 0.75.into())), None);
        assert_eq!(cci.next((2.0.into(), 1.0.into(), 1.5.into())), None);
        let result = cci.next((3.0.into(), 1.5.into(), 2.25.into())).unwrap().round_dp(2);
        assert_eq!(result, IndicatorValue::from(100.0));
    }

    #[test]
    fn test_cci_next_chunk() {
        let mut cci = CommodityChannelIndex::new(3);
        let result = cci.next_chunk(&[
            (1.0.into(), 0.5.into(), 0.75.into()),
            (2.0.into(), 1.0.into(), 1.5.into()),
            (3.0.into(), 1.5.into(), 2.25.into()),
        ]);
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(100.0));
    }

    #[test]
    fn test_cci_reset() {
        let mut cci = CommodityChannelIndex::new(3);
        cci.next((1.0.into(), 0.5.into(), 0.75.into()));
        cci.next((2.0.into(), 1.0.into(), 1.5.into()));
        cci.reset();
        assert_eq!(cci.next((3.0.into(), 1.5.into(), 2.25.into())), None);
    }

    #[test]
    fn test_cci_with_large_data() {
        let mut cci = CommodityChannelIndex::new(100);
        let data: Vec<(IndicatorValue, IndicatorValue, IndicatorValue)> = (1..=1000).map(|x| {
            let val = IndicatorValue::from(x as f64);
            (val, val / 2.0.into(), val * 0.75.into())
        }).collect();
        let result = cci.next_chunk(&data);

        assert!(result.is_some());
        assert_eq!(result.unwrap().round_dp(2), IndicatorValue::from(132.0));
    }

    #[test]
    fn test_cci_all_same_values() {
        let mut cci = CommodityChannelIndex::new(3);
        assert_eq!(
            cci.next_chunk(&[
                (2.0.into(), 2.0.into(), 2.0.into()),
                (2.0.into(), 2.0.into(), 2.0.into()),
                (2.0.into(), 2.0.into(), 2.0.into()),
            ]).unwrap().round_dp(2),
            IndicatorValue::from(0.0)
        );
    }

    #[test]
    fn test_cci_with_zeros() {
        let mut cci = CommodityChannelIndex::new(3);
        assert_eq!(
            cci.next_chunk(&[
                (0.0.into(), 0.0.into(), 0.0.into()),
                (0.0.into(), 0.0.into(), 0.0.into()),
                (0.0.into(), 0.0.into(), 0.0.into()),
            ]).unwrap().round_dp(2),
            IndicatorValue::from(0.0)
        );
    }
}
