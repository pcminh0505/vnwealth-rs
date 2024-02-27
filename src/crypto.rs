use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

//---------- BASE URLs ----------//
const BINANCE_BASE_URL: &str = "https://api.binance.com/api/v3";
const COINGECKO_BASE_URL: &str = "https://api.coingecko.com/api/v3/";

//---------- Struct Definition ----------//
//---------- Binance 24h Ticker ----------//
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

//---------- CoinGecko Market ----------//
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinGeckoItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "current_price")]
    pub current_price: f64,
    #[serde(rename = "market_cap")]
    pub market_cap: i64,
    #[serde(rename = "market_cap_rank")]
    pub market_cap_rank: i64,
    #[serde(rename = "fully_diluted_valuation")]
    pub fully_diluted_valuation: Option<i64>,
    #[serde(rename = "total_volume")]
    pub total_volume: i64,
    #[serde(rename = "high_24h")]
    pub high_24h: f64,
    #[serde(rename = "low_24h")]
    pub low_24h: f64,
    #[serde(rename = "price_change_24h")]
    pub price_change_24h: f64,
    #[serde(rename = "price_change_percentage_24h")]
    pub price_change_percentage_24h: f64,
    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change_24h: f64,
    #[serde(rename = "market_cap_change_percentage_24h")]
    pub market_cap_change_percentage_24h: f64,
    #[serde(rename = "circulating_supply")]
    pub circulating_supply: f64,
    #[serde(rename = "total_supply")]
    pub total_supply: Option<f64>,
    #[serde(rename = "max_supply")]
    pub max_supply: Option<f64>,
    pub ath: f64,
    #[serde(rename = "ath_change_percentage")]
    pub ath_change_percentage: f64,
    #[serde(rename = "ath_date")]
    pub ath_date: String,
    pub atl: f64,
    #[serde(rename = "atl_change_percentage")]
    pub atl_change_percentage: f64,
    #[serde(rename = "atl_date")]
    pub atl_date: String,
    pub roi: Option<Roi>,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "sparkline_in_7d")]
    pub sparkline_in_7d: SparklineIn7d,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roi {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SparklineIn7d {
    pub price: Vec<f64>,
}

//---------- Simplified Response ----------//
#[derive(Debug, Deserialize)]
pub struct TokenPriceChange {
    pub last_price: f64,
    pub price_change_percent: f64,
}

//---------- Functions ----------//
pub async fn _get_ticker_change(
    symbols: Vec<String>,
) -> Result<HashMap<String, TokenPriceChange>, Box<dyn std::error::Error>> {
    let symbols_str = format!(
        "[{}]",
        symbols
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(",")
    );
    let query_params = format!("symbols={}", symbols_str);
    let url = format!(r#"{}/ticker/24hr?{}"#, BINANCE_BASE_URL, query_params);

    // println!("{url:#?}");

    let resp = reqwest::get(url)
        .await?
        .json::<Vec<BinanceTicker24h>>()
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

// Note: CoinGecko API needs UserAgent to bypass blocking
pub async fn _get_coingecko_market(
) -> Result<HashMap<String, TokenPriceChange>, Box<dyn std::error::Error>> {
    let query_params = format!(
        "vs_currency={}&order={}&per_page={}&page={}&sparkline={}&price_change_percentage={}",
        "usd", "market_cap_desc", 100, 1, true, "7d"
    );
    let url = format!("{}/coins/markets?{}", COINGECKO_BASE_URL, query_params);

    // println!("{url:#?}");
    let client = reqwest::Client::new();
    // Create a custom User-Agent string
    let custom_user_agent = "MyCustomUserAgent/1.0";
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, custom_user_agent.parse().unwrap());

    let resp = client.get(url).headers(headers).send().await?;

    if !resp.status().is_success() {
        let error = format!(
            "Error {} while getting data {:?}",
            &resp.status(),
            resp.text().await.unwrap()
        );

        Err(error.into())
    } else {
        let res_data = resp.json::<Vec<CoinGeckoItem>>().await?;
        // println!("{res_data:#?}");

        let tickers: HashMap<String, TokenPriceChange> = res_data
            .into_iter()
            .map(|a| {
                (
                    a.symbol,
                    TokenPriceChange {
                        last_price: a.current_price,
                        price_change_percent: a.price_change_24h,
                    },
                )
            })
            .collect();

        Ok(tickers)
    }
}
