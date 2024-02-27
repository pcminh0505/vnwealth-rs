use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

//---------- BASE URLs ----------//
const TCBS_BASE_URL: &str = "https://apipubaws.tcbs.com.vn/tcanalysis/v1/";
const VNDIRECT_BASE_URL: &str = "https://finfo-api.vndirect.com.vn/v4/";

pub enum VNStockPlatform {
    TCBS,
    VNDIRECT(String, Option<String>),
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

// Document: https://github.com/thinh-vu/vnstock/blob/main/vnstock/trading.py
// Update 24 Feb 2024: TCBS Tickers API seems dead :(
pub async fn _get_ticker_change(
    symbols: Vec<String>,
    platform: VNStockPlatform,
) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    match platform {
        VNStockPlatform::TCBS => _get_tcbs_ticker(symbols).await,
        VNStockPlatform::VNDIRECT(now, begin) => _get_vndirect_ticker(symbols, now, begin).await,
    }
}

async fn _get_tcbs_ticker(
    symbols: Vec<String>,
) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    let query_params = format!("tickers={}&fType=TICKERS", symbols.join(",").to_uppercase(),);

    let url = format!("{}/rating/detail/council?{}", TCBS_BASE_URL, query_params);

    // println!("{url:#?}");

    let resp = reqwest::get(url).await?.json::<Vec<TCBSTicker>>().await?;
    // println!("{resp:#?}");

    let tickers: HashMap<String, i64> = resp.into_iter().map(|a| (a.ticker, a.price)).collect();

    Ok(tickers)
}

async fn _get_vndirect_ticker(
    symbols: Vec<String>,
    end_date: String,
    start_date: Option<String>,
) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    let query_params = format!(
        "q=code:{}~date:gte:{}~date:lte:{}",
        symbols.join(",").to_uppercase(),
        start_date.unwrap_or(end_date.clone()),
        end_date
    );

    let url = format!("{}stock_prices?{}", VNDIRECT_BASE_URL, query_params);

    // println!("{url:#?}");

    let resp = reqwest::get(url).await?.json::<VNDirectResponse>().await?;

    println!("{resp:#?}");

    let tickers: HashMap<String, i64> = resp
        .data
        .into_iter()
        .map(|a| (a.code, a.basic_price as i64 * 1000))
        .collect();

    Ok(tickers)
}
