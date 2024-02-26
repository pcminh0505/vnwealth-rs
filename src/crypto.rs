use std::collections::HashMap;

use serde::Deserialize;

//---------- BASE URLs ----------//
const BINANCE_BASE_URL: &str = "https://api.binance.com/api/v3";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceTicker24hRaw {
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
#[derive(Debug, Deserialize)]
pub struct TokenPriceChange {
    pub last_price: f64,
    pub price_change_percent: f64,
}

pub async fn _get_ticker_change(
    symbols: Vec<String>,
) -> Result<HashMap<String, TokenPriceChange>, Box<dyn std::error::Error>> {
    let string = format!(
        "[{}]",
        symbols
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(",")
    );
    let url = format!(
        r#"{}/ticker/24hr?symbols={}"#,
        BINANCE_BASE_URL,
        string.to_uppercase()
    );

    // println!("{url:#?}");

    let resp = reqwest::get(url)
        .await?
        .json::<Vec<BinanceTicker24hRaw>>()
        .await?;
    // println!("{resp:#?}");

    let tickers: HashMap<String, TokenPriceChange> = resp
        .into_iter()
        .map(|a| {
            (
                a.symbol.replace("USDT", ""),
                TokenPriceChange {
                    last_price: a.last_price.parse::<f64>().unwrap(),
                    price_change_percent: a.price_change_percent.parse::<f64>().unwrap(),
                },
            )
        })
        .collect();

    Ok(tickers)
}
