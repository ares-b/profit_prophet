use async_trait::async_trait;

use connector::http::HttpClientError;

use technical_analysis::OHLCV;

mod pump_fun;

#[async_trait]
pub trait TradingPlatform {
    async fn get_latest_candlestick(self, ticker: String, timeframe: String) -> Result<OHLCV, HttpClientError>;
    async fn get_candlesticks(self, ticker: String, timeframe: String, candlesticks_number: u8) -> Result<Vec<OHLCV>, HttpClientError>;
}
