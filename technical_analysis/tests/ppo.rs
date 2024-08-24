#[cfg(test)]
mod tests {
    use technical_analysis::indicators::{Indicator, PercentagePriceOscillator, PPOOutput};
    use technical_analysis::IndicatorValue;

    #[test]
    fn test_default_ppo() {
        let mut ppo = PercentagePriceOscillator::default();

        // Provide a sequence of values where the expected outputs are known.
        let inputs = vec![
            IndicatorValue::from(100.0), IndicatorValue::from(101.0), IndicatorValue::from(102.0),
            IndicatorValue::from(103.0), IndicatorValue::from(104.0), IndicatorValue::from(105.0),
            IndicatorValue::from(106.0), IndicatorValue::from(107.0), IndicatorValue::from(108.0),
            IndicatorValue::from(109.0), IndicatorValue::from(110.0), IndicatorValue::from(111.0),
            IndicatorValue::from(112.0), IndicatorValue::from(113.0), IndicatorValue::from(114.0),
            IndicatorValue::from(115.0), IndicatorValue::from(116.0), IndicatorValue::from(117.0),
            IndicatorValue::from(118.0), IndicatorValue::from(119.0), IndicatorValue::from(120.0),
            IndicatorValue::from(121.0), IndicatorValue::from(122.0), IndicatorValue::from(123.0),
            IndicatorValue::from(124.0), IndicatorValue::from(125.0), IndicatorValue::from(126.0),
            IndicatorValue::from(127.0), IndicatorValue::from(128.0), IndicatorValue::from(129.0),
            IndicatorValue::from(130.0), IndicatorValue::from(131.0), IndicatorValue::from(132.0),
            IndicatorValue::from(133.0), IndicatorValue::from(134.0),
        ];

        // We won't expect any output until we have at least 34 data points.
        let expected_results = vec![
            None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None,
            None, None, None,
            Some(PPOOutput {
                ppo_value: IndicatorValue::from(4.97),
                signal_value: IndicatorValue::from(4.84),
                histogram_value: IndicatorValue::from(0.13),
            }),
        ];

        for (input, expected) in inputs.into_iter().zip(expected_results.into_iter()) {
            let result = ppo.next(input).map(|output| PPOOutput {
                ppo_value: output.ppo_value.round_dp(2),
                signal_value: output.signal_value.round_dp(2),
                histogram_value: output.histogram_value.round_dp(2),
            });
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_reset_ppo() {
        let mut ppo = PercentagePriceOscillator::new(12, 26, 9);

        let inputs = vec![
            IndicatorValue::from(100.0),
            IndicatorValue::from(101.0),
            IndicatorValue::from(102.0),
        ];

        for input in &inputs {
            ppo.next(*input);
        }

        // Reset the PPO
        ppo.reset();

        // After reset, it should behave as if it was just initialized
        let result = ppo.next(IndicatorValue::from(100.0));
        assert_eq!(result, None);
    }

    #[test]
    fn test_ppo_edge_case_small_periods() {
        let mut ppo = PercentagePriceOscillator::new(1, 2, 1);

        let inputs = vec![
            IndicatorValue::from(100.0),
            IndicatorValue::from(102.0),
        ];

        let expected_results = vec![
            None, // First input should return None, as long EMA requires at least 2 values.
            Some(PPOOutput {
                ppo_value: IndicatorValue::from(0.66),
                signal_value: IndicatorValue::from(0.66),
                histogram_value: IndicatorValue::from(0.00),
            }),
        ];

        for (input, expected) in inputs.into_iter().zip(expected_results.into_iter()) {
            let result = ppo.next(input).map(|output| PPOOutput {
                ppo_value: output.ppo_value.round_dp(2),
                signal_value: output.signal_value.round_dp(2),
                histogram_value: output.histogram_value.round_dp(2),
            });
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_ppo_edge_case_identical_inputs() {
        let mut ppo = PercentagePriceOscillator::default();

        let inputs = vec![
            IndicatorValue::from(100.0); 50
        ];

        for i in 0..34 {
            ppo.next(inputs[i]);
        }
        for i in 34..inputs.len() {
            assert_eq!(ppo.next(inputs[i]).map(|output| PPOOutput {
                ppo_value: output.ppo_value.round_dp(2),
                signal_value: output.signal_value.round_dp(2),
                histogram_value: output.histogram_value.round_dp(2),
            }), Some(PPOOutput {
                ppo_value: IndicatorValue::from(0.0),
                signal_value: IndicatorValue::from(0.0),
                histogram_value: IndicatorValue::from(0.0),
            }));
        }
        
    }
}
