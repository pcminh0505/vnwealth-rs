use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

//---------- BASE URLs ----------//
const TCBS_BASE_URL: &str = "https://apipubaws.tcbs.com.vn/tcanalysis/v1/";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TCBSRawTickers {
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

// Document: https://github.com/thinh-vu/vnstock/blob/main/vnstock/trading.py
// Seems like API can only be called within trading time (8:00 - 17:00)
pub async fn _get_ticker_change(
    symbols: Vec<String>,
) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    let string = symbols.join(",");

    let url = format!(
        "{}/rating/detail/council?tickers={}&fType=TICKERS",
        TCBS_BASE_URL,
        string.to_uppercase()
    );

    println!("{url:#?}");

    let resp = reqwest::get(url)
        .await?
        .json::<Vec<TCBSRawTickers>>()
        .await?;
    println!("{resp:#?}");

    let tickers: HashMap<String, i64> = resp.into_iter().map(|a| (a.ticker, a.price)).collect();

    Ok(tickers)
}
