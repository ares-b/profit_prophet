#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{DonchianChannels, Indicator};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_donchian_channels_next() {
        let mut atr = DonchianChannels::new(14);
        let data = vec![
            (22.27, 21.19), (22.29, 21.21), (22.31, 21.23),
            (22.33, 21.25), (22.35, 21.27), (22.37, 21.29),
            (22.39, 21.31), (22.41, 21.33), (22.43, 21.35),
            (22.45, 21.37), (22.47, 21.39), (22.49, 21.41),
            (22.51, 21.43), (22.53, 21.45),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        let result = atr.next_chunk(&data).unwrap();
        assert_eq!(result.upper_band.round_dp(2), IndicatorValue::from(22.53));
        assert_eq!(result.middle_band.round_dp(2), IndicatorValue::from(21.86));
        assert_eq!(result.lower_band.round_dp(2), IndicatorValue::from(21.19));
    }

    #[test]
    fn test_donchian_channels_with_insufficient_data() {
        let mut donchian = DonchianChannels::new(20);

        // Only 10 data points, not enough to form a complete Donchian Channel with a period of 20
        for i in 1..11 {
            let result = donchian.next((IndicatorValue::from(10.0 + i as f64), IndicatorValue::from(5.0 + i as f64 / 2.0)));
            assert_eq!(result, None);
        }
    }

    #[test]
    fn test_donchian_channels_reset() {
        let mut donchian = DonchianChannels::new(14);

        // Fill buffer with initial data
        let data = vec![
            (22.27, 21.19), (22.29, 21.21), (22.31, 21.23),
            (22.33, 21.25), (22.35, 21.27), (22.37, 21.29),
            (22.39, 21.31), (22.41, 21.33), (22.43, 21.35),
            (22.45, 21.37), (22.47, 21.39), (22.49, 21.41),
            (22.51, 21.43), (22.53, 21.45),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        donchian.next_chunk(&data);

        // Reset the Donchian Channels indicator
        donchian.reset();

        // After reset, no result should be produced until the buffer is refilled
        assert_eq!(donchian.next((IndicatorValue::from(22.55), IndicatorValue::from(21.47))), None);
    }

    #[test]
    fn test_donchian_channels_full_buffer_rollover() {
        let mut donchian = DonchianChannels::new(3);

        // Fill buffer
        donchian.next((IndicatorValue::from(22.27), IndicatorValue::from(21.19)));
        donchian.next((IndicatorValue::from(22.29), IndicatorValue::from(21.21)));
        let output = donchian.next((IndicatorValue::from(22.31), IndicatorValue::from(21.23))).unwrap();

        assert_eq!(output.upper_band.round_dp(2), IndicatorValue::from(22.31));
        assert_eq!(output.middle_band.round_dp(2), IndicatorValue::from(21.75));
        assert_eq!(output.lower_band.round_dp(2), IndicatorValue::from(21.19));

        // Add a new value and roll over the buffer
        let output = donchian.next((IndicatorValue::from(22.33), IndicatorValue::from(21.25))).unwrap();
        assert_eq!(output.upper_band.round_dp(2), IndicatorValue::from(22.33));
        assert_eq!(output.lower_band.round_dp(2), IndicatorValue::from(21.21));
        assert_eq!(output.middle_band.round_dp(2), IndicatorValue::from(21.77));
    }

    #[test]
    fn test_donchian_channels_next_chunk() {
        let mut donchian = DonchianChannels::new(3);

        let data = vec![
            (22.27, 21.19), (22.29, 21.21), (22.31, 21.23),
        ].into_iter().map(|(h, l)| (IndicatorValue::from(h), IndicatorValue::from(l))).collect::<Vec<_>>();

        let result = donchian.next_chunk(&data).unwrap();
        assert_eq!(result.upper_band.round_dp(2), IndicatorValue::from(22.31));
        assert_eq!(result.middle_band.round_dp(2), IndicatorValue::from(21.75));
        assert_eq!(result.lower_band.round_dp(2), IndicatorValue::from(21.19));
    }

    #[test]
    fn test_donchian_channels_large_period() {
        let mut donchian = DonchianChannels::new(50);

        let data = (1..=100).map(|i| (
            IndicatorValue::from(i as f64), 
            IndicatorValue::from(i as f64 / 2.0)
        )).collect::<Vec<_>>();
        let result = donchian.next_chunk(&data).unwrap();

        println!("Upper Band: {:?}", result.upper_band);
        println!("Middle Band: {:?}", result.middle_band);
        println!("Lower Band: {:?}", result.lower_band);
        println!("Current Min: {:?}", donchian.current_min);
        println!("Current Max: {:?}", donchian.current_max);
        println!("Low Buffer: {:?}", donchian.low_buffer);
        println!("High Buffer: {:?}", donchian.high_buffer);
        
        assert_eq!(result.upper_band.round_dp(2), IndicatorValue::from(100.0));
        assert_eq!(result.middle_band.round_dp(2), IndicatorValue::from(62.75));
        assert_eq!(result.lower_band.round_dp(2), IndicatorValue::from(25.5));
    }
}
