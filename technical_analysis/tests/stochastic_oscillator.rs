#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, LowOutput, StochasticOscillator, StochasticOutput};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_stochastic_oscillator_basic() {
        let mut stoch = StochasticOscillator::new(3, 3);

        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(5.0), IndicatorValue::from(7.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(15.0), IndicatorValue::from(6.0), IndicatorValue::from(9.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(20.0), IndicatorValue::from(5.0), IndicatorValue::from(15.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(25.0), IndicatorValue::from(10.0), IndicatorValue::from(20.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(25.0), IndicatorValue::from(10.0), IndicatorValue::from(20.0)));
        
        let output = output.unwrap();
        assert_eq!(output.k.round_dp(2), IndicatorValue::from(75.0));
        assert_eq!(output.d.round_dp(2), IndicatorValue::from(72.22)); 
    }

    #[test]
    fn test_stochastic_oscillator_identical_high_low() {
        let mut stoch = StochasticOscillator::new(3, 3);

        // All high and low values are the same, causing the range to be 0
        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(10.0), IndicatorValue::from(10.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(10.0), IndicatorValue::from(10.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(10.0), IndicatorValue::from(10.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(10.0), IndicatorValue::from(10.0)));
        assert!(output.is_none());

        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(10.0), IndicatorValue::from(10.0)));

        if let Some(output) = output {
            assert_eq!(output.k.round_dp(2), IndicatorValue::from(50.0));
            assert_eq!(output.d.round_dp(2), IndicatorValue::from(50.0));
        }
    }

    #[test]
    fn test_stochastic_oscillator_reset() {
        let mut stoch = StochasticOscillator::new(3, 3);

        stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(5.0), IndicatorValue::from(7.0)));
        stoch.next((IndicatorValue::from(15.0), IndicatorValue::from(6.0), IndicatorValue::from(9.0)));
        stoch.next((IndicatorValue::from(20.0), IndicatorValue::from(5.0), IndicatorValue::from(15.0)));
        stoch.next((IndicatorValue::from(20.0), IndicatorValue::from(5.0), IndicatorValue::from(15.0)));
        let output = stoch.next((IndicatorValue::from(20.0), IndicatorValue::from(5.0), IndicatorValue::from(15.0)));
        assert!(output.is_some());
        stoch.reset();

        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(5.0), IndicatorValue::from(7.0)));
        assert!(output.is_none());


    }

    #[test]
    fn test_stochastic_oscillator_minimal_period() {
        let mut stoch = StochasticOscillator::new(1, 1);

        // With period of 1, each call should produce an output immediately after the first call
        let output = stoch.next((IndicatorValue::from(10.0), IndicatorValue::from(5.0), IndicatorValue::from(7.0)));
        assert!(output.is_some());

        if let Some(output) = output {
            assert_eq!(output.k.round_dp(2), IndicatorValue::from(40.0));
            assert_eq!(output.d.round_dp(2), IndicatorValue::from(40.0));
        }
    }

    #[test]
    fn test_stochastic_oscillator_next_chunk() {
        let mut stoch = StochasticOscillator::new(3, 3);

        let data = [
            (IndicatorValue::from(127.01), IndicatorValue::from(125.36), IndicatorValue::from(126.63)),
            (IndicatorValue::from(126.59), IndicatorValue::from(124.93), IndicatorValue::from(125.54)),
            (IndicatorValue::from(126.82), IndicatorValue::from(125.72), IndicatorValue::from(126.30)),
            (IndicatorValue::from(127.14), IndicatorValue::from(126.09), IndicatorValue::from(126.82)),
            (IndicatorValue::from(128.23), IndicatorValue::from(126.64), IndicatorValue::from(127.91)),
        ];

        let output = stoch.next_chunk(&data).unwrap();

        assert_eq!(output.k.round_dp(2), IndicatorValue::from(87.25));
        assert_eq!(output.d.round_dp(2), IndicatorValue::from(79.55));
    }

}
