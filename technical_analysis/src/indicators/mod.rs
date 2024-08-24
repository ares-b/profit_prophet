mod aroon;
mod atr;
mod bollinger_bands;
mod cmf;
mod cmo;
mod donchian_channels;
mod ema;
mod high_low;
mod keltner_channels;
mod macd;
mod mean_abs_stdev;
mod median_abs_stdev; 
mod obv;
mod parabolic_sar;
mod ppo;
mod roc;
mod rsi;
mod sma;
mod smm; 
mod stdev;
mod stochastic_oscillator;
mod vwap;
mod cci;

pub use aroon::{Aroon, AroonOutput}; 
pub use atr::AverageTrueRange;
pub use bollinger_bands::{BollingerBands, BollingerBandsOutput};
pub use cmf::ChaikinMoneyFlow;
pub use cmo::ChandeMomentumOscillator;
pub use donchian_channels::{DonchianChannels, DonchianChannelsOutput};
pub use ema::ExponentialMovingAverage;
pub use high_low::{High, Low, HighLow, HighOutput, LowOutput, HighLowOutput};
pub use keltner_channels::{KeltnerChannels, KeltnerChannelsOutput};
pub use macd::{MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceOutput};
pub use mean_abs_stdev::MeanAbsDev;
pub use median_abs_stdev::MedianAbsoluteStandardDeviation; 
pub use obv::OnBalanceVolume;
pub use parabolic_sar::ParabolicSAR;
pub use ppo::{PercentagePriceOscillator, PPOOutput};
pub use roc::RateOfChange;
pub use rsi::RelativeStrengthIndex;
pub use sma::SimpleMovingAverage;
pub use smm::SimpleMovingMedian;
pub use stdev::StandardDeviation;
pub use stochastic_oscillator::{StochasticOscillator, StochasticOutput};
pub use vwap::VolumeWeightedAveragePrice;
pub use cci::CommodityChannelIndex;

pub trait Indicator {
    type Output;
    type Input;
    
    fn next(&mut self, input: Self::Input) -> Self::Output;
    fn next_chunk(&mut self, input: &[Self::Input]) -> Self::Output;
    fn reset(&mut self);
}
