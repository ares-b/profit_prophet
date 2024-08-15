#![feature(test)]

extern crate test;
use test::Bencher;
use technical_analysis::indicators::{
    Indicator, SimpleMovingAverage, StandardDeviation, RateOfChange, RelativeStrengthIndex,
    BollingerBands, Aroon, AverageTrueRange, ChaikinMoneyFlow, ChandeMomentumOscillator,
    DonchianChannels, ExponentialMovingAverage, KeltnerChannels,
    MovingAverageConvergenceDivergence, MeanAbsDev, MedianAbsDev, OnBalanceVolume,
    ParabolicSAR, PercentagePriceOscillator, VolumeWeightedAveragePrice,
    StochasticOscillator, WoodiesCCI,
};
use technical_analysis::IndicatorValue;

// Helper to generate random data for benchmarks
fn generate_random_data(seed: u64, len: usize) -> Vec<IndicatorValue> {
    let mut state = seed;
    let mut data = Vec::with_capacity(len);
    for _ in 0..len {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let value = (state >> 32) as f64 / (1u64 << 32) as f64 * 100.0;
        data.push(IndicatorValue::from(value));
    }
    data
}

// Generic benchmark function for indicators
fn bench_indicator<I, T>(b: &mut Bencher, input_data: &[I], mut indicator: T)
where
    T: Indicator<Input = I>,
    I: Copy,
{
    let mut iter = input_data.iter().cycle();

    // Prime the indicator with initial values
    for _ in 0..50 {
        indicator.next(*iter.next().unwrap());
    }

    b.iter(|| {
        // Measure performance of the indicator's next value computation
        indicator.next(*iter.next().unwrap())
    });
}

// Generate tuple data for indicators that require multiple input values
fn generate_tuple_data2(seed: u64, len: usize) -> Vec<(IndicatorValue, IndicatorValue)> {
    let data = generate_random_data(seed, len);
    data.windows(2)
        .map(|w| (w[0], w[1]))
        .collect()
}

fn generate_tuple_data3(seed: u64, len: usize) -> Vec<(IndicatorValue, IndicatorValue, IndicatorValue)> {
    let data = generate_random_data(seed, len);
    data.windows(3)
        .map(|w| (w[0], w[1], w[2]))
        .collect()
}

fn generate_tuple_data4(seed: u64, len: usize) -> Vec<(IndicatorValue, IndicatorValue, IndicatorValue, IndicatorValue)> {
    let data = generate_random_data(seed, len);
    data.windows(4)
        .map(|w| (w[0], w[1], w[2], w[3]))
        .collect()
}

// Reusable macro for benchmarks
macro_rules! create_bench {
    ($name:ident, $indicator:expr, $data:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let data = $data;
            let indicator = $indicator;
            bench_indicator(b, &data, indicator);
        }
    };
}

// Benchmark Definitions
create_bench!(bench_sma_p14, SimpleMovingAverage::new(14), generate_random_data(12345, 1000));
create_bench!(bench_stdev_p14, StandardDeviation::new(14), generate_random_data(12345, 1000));
create_bench!(bench_roc, RateOfChange::new(12), generate_random_data(12345, 1000));
create_bench!(bench_rsi, RelativeStrengthIndex::new(14), generate_random_data(12345, 1000));
create_bench!(bench_bollinger_bands, BollingerBands::new(20, 2.0), generate_random_data(12345, 1000));
create_bench!(bench_aroon, Aroon::new(14), generate_tuple_data2(12345, 1000));
create_bench!(bench_atr, AverageTrueRange::new(14), generate_tuple_data3(12345, 1000));
create_bench!(bench_cmf, ChaikinMoneyFlow::new(20), generate_tuple_data4(12345, 1000));
create_bench!(bench_cmo, ChandeMomentumOscillator::new(14), generate_random_data(12345, 1000));
create_bench!(bench_donchian_channels, DonchianChannels::new(20), generate_tuple_data2(12345, 1000));
create_bench!(bench_ema, ExponentialMovingAverage::new(12), generate_random_data(12345, 1000));
create_bench!(bench_keltner_channels, KeltnerChannels::new(20, 1.5), generate_tuple_data3(12345, 1000));
create_bench!(bench_macd, MovingAverageConvergenceDivergence::new(12, 26, 9), generate_random_data(12345, 1000));
create_bench!(bench_mean_abs_dev, MeanAbsDev::new(14), generate_random_data(12345, 1000));
create_bench!(bench_median_abs_dev, MedianAbsDev::new(14), generate_random_data(12345, 1000));
create_bench!(bench_obv, OnBalanceVolume::new(), generate_tuple_data2(12345, 1000));
create_bench!(bench_parabolic_sar, ParabolicSAR::default(), generate_tuple_data2(12345, 1000));
create_bench!(bench_ppo, PercentagePriceOscillator::new(12, 26, 9), generate_random_data(12345, 1000));
create_bench!(bench_vwap, VolumeWeightedAveragePrice::default(), generate_tuple_data4(12345, 1000));
create_bench!(bench_stochastic_oscillator, StochasticOscillator::default(), generate_tuple_data3(12345, 1000));
create_bench!(bench_woodies_cci, WoodiesCCI::new(14), generate_tuple_data3(12345, 1000));
