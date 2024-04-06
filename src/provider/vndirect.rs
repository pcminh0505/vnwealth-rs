use super::DataProvider;
use crate::defaults::VNDIRECT_BASE_URL;
use anyhow::Result;
use chrono::{Datelike, Duration, Local};
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};

pub struct VNDirectDataProvider {
    base_url: String,
}

impl DataProvider for VNDirectDataProvider {
    fn new() -> Self {
        VNDirectDataProvider {
            base_url: VNDIRECT_BASE_URL.to_string(),
        }
    }

    async fn fetch_asset_price(&self, symbol: Option<String>) -> Result<f32> {
        // Get today's date
        let today = Local::now();
        let window = today - Duration::days(5);

        // Format date as YYYY-MM-DD
        let start_date = format!(
            "{:04}-{:02}-{:02}",
            window.year(),
            window.month(),
            window.day()
        );

        let end_date = format!(
            "{:04}-{:02}-{:02}",
            today.year(),
            today.month(),
            today.day()
        );

        // Note: This API can fetch multiple symbols as well: symbols.join(",").to_uppercase(),
        // but will require filtering tickers
        let query_params = format!(
            "q=code:{}~date:gte:{}~date:lte:{}",
            symbol.unwrap().to_uppercase(),
            start_date,
            end_date
        );

        let url = format!("{}stock_prices?{}", self.base_url.clone(), query_params);
        // println!("{url:#?}");

        let client = reqwest::Client::new();
        // Create a custom User-Agent string
        let custom_user_agent = "MyCustomUserAgent/1.0";
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, custom_user_agent.parse().unwrap());

        let resp = client.get(url).headers(headers).send().await?;

        let res_data = resp.json::<VNDirectResponse>().await?;

        // Get the last elenment in data list (most recent price)
        let current_price = res_data.data.last().unwrap().basic_price as i64 * 1000;

        Ok(current_price as f32)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VNDirectTicker {
    pub code: String,
    pub date: String,
    pub time: String,
    pub floor: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub basic_price: f64,
    pub ceiling_price: f64,
    pub floor_price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub average: f64,
    pub ad_open: f64,
    pub ad_high: f64,
    pub ad_low: f64,
    pub ad_close: f64,
    pub ad_average: f64,
    pub nm_volume: f64,
    pub nm_value: f64,
    pub pt_volume: f64,
    pub pt_value: f64,
    pub change: f64,
    pub ad_change: f64,
    pub pct_change: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VNDirectResponse {
    pub data: Vec<VNDirectTicker>,
    pub current_page: i64,
    pub size: i64,
    pub total_elements: i64,
    pub total_pages: i64,
}
