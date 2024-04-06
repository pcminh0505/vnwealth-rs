use super::DataProvider;
use crate::defaults::BINANCE_BASE_URL;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct BinanceDataProvider {
    base_url: String,
}

impl DataProvider for BinanceDataProvider {
    fn new() -> Self {
        BinanceDataProvider {
            base_url: BINANCE_BASE_URL.to_string(),
        }
    }

    async fn fetch_asset_price(&self, symbol: Option<String>) -> Result<f32> {
        let query_params = format!("symbol={}", symbol.unwrap().to_uppercase() + "USDT");
        let url = format!(r#"{}/ticker/24hr?{}"#, self.base_url.clone(), query_params);

        let resp = reqwest::get(url).await?.json::<BinanceTicker24h>().await?;

        Ok(resp.last_price.parse::<f32>().unwrap())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceTicker24h {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub prev_close_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: i64,
}
