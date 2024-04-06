use super::DataProvider;
use crate::defaults::TCBS_BASE_URL;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct TCBSDataProvider {
    base_url: String,
}

impl DataProvider for TCBSDataProvider {
    fn new() -> Self {
        TCBSDataProvider {
            base_url: TCBS_BASE_URL.to_string(),
        }
    }

    async fn fetch_asset_price(&self, symbol: Option<String>) -> Result<f32> {
        let query_params = format!("ticker={}&fType=TICKER", symbol.unwrap().to_uppercase(),);

        let url = format!(
            "{}/rating/detail/single?{}",
            self.base_url.clone(),
            query_params
        );

        let resp = reqwest::get(url).await?.json::<TCBSTicker>().await?;

        Ok(resp.price as f32)
    }

    // For multiple: use {}/rating/detail/council?tickers=<list>&fType=TICKERS
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TCBSTicker {
    pub ticker: String,
    pub marcap: i64,
    pub price: i64,
    pub number_of_days: i64,
    pub price_to_earning: f64,
    pub peg: f64,
    pub price_to_book: f64,
    pub value_before_ebitda: Value,
    pub dividend: f64,
    pub roe: f64,
    pub roa: f64,
    pub ebit_on_interest: Value,
    pub book_value_per_share: Value,
    pub interest_margin: f64,
    pub bad_debt_percentage: f64,
    pub current_payment: Value,
    pub quick_payment: Value,
    pub gross_profit_margin: Value,
    pub operating_profit_margin: Value,
    pub post_tax_margin: Value,
    pub debt_on_equity: f64,
    pub debt_on_ebitda: Value,
    pub income5year: f64,
    pub sale5year: f64,
    pub income1quarter: f64,
    pub sale1quarter: f64,
    pub next_income: f64,
    pub next_sale: f64,
    pub rsi: f64,
    pub rs: f64,
}
