use std::{collections::HashMap, hash::Hash};
use async_trait::async_trait;

use serde::Deserialize;

use technical_analysis::{IndicatorValue, OHLCV};
use connector::http::{HttpClient, HttpRequest, HttpClientError};

use crate::account::SolanaAccount;
use super::TradingPlatform;

#[derive(Deserialize)]
struct PumpFunCandlestickResponse {
    open: IndicatorValue,
    high: IndicatorValue,
    low: IndicatorValue,
    close: IndicatorValue,
    volume: IndicatorValue,
}

pub struct PumpFun {
    account: SolanaAccount,
    http_client: HttpClient
}

impl PumpFun {
    const PUMP_FUN_API_URL: &str = "https://frontend-api.pump.fun";

    pub fn new(account: SolanaAccount) -> Self {
        PumpFun {
            account,
            http_client: HttpClient::new()
        }
    }
}


#[async_trait]
impl TradingPlatform for PumpFun {

    async fn get_latest_candlestick(self, ticker: String, timeframe: String) -> Result<OHLCV, HttpClientError> {
        match self.get_candlesticks(ticker, timeframe, 1).await {
            Ok(candlesticks) => Ok(candlesticks[0]),
            Err(err) => Err(err)
        }
    }
    
    async fn get_candlesticks(self, ticker: String, timeframe: String, candlesticks_number: u8) -> Result<Vec<OHLCV>, HttpClientError> {
        match self.http_client.request::<String, Vec<PumpFunCandlestickResponse>>(HttpRequest{
            method: "GET".to_string(),
            url: format!("{}/{}", Self::PUMP_FUN_API_URL.to_owned(), ticker),
            body: Some("".to_string()),
            headers: Some(HashMap::from([
                ("Content-Type".to_string(), "application/json".to_string()),
            ])),
            query_params: Some(HashMap::from([
                ("offset".to_string(), 0.to_string()),
                ("limit".to_string(), candlesticks_number.to_string()),
                ("timeframe".to_string(), timeframe),
            ])),
            timeout_duration: None,
        }).await {
            Ok(response) => match response.body {
                Some(candlesticks) => Ok(
                    candlesticks.into_iter().map(|candlestick| 
                        OHLCV {
                            open: candlestick.open,
                            high: candlestick.high,
                            low: candlestick.low,
                            close: candlestick.close,
                            volume: candlestick.volume,
                        }
                    ).collect::<Vec<OHLCV>>()
                ),
                None => Err(HttpClientError::DeserializeError("".to_string()))
            } 
            Err(err) => Err(err)
        }
    }
}