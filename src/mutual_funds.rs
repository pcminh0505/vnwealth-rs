use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//---------- BASE URLs ----------//
const DRAGONCAPITAL_BASE_URL: &str = "https://api.dragoncapital.com.vn/nav/getLatestValue.php";
const VINACAPITAL_BASE_URL: &str = "https://wm.vinacapital.com/wp-admin/admin-ajax.php";

//---------- DragonCapital ----------//
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragonCapitalNAV {
    #[serde(rename = "trade_code")]
    pub trade_code: String,
    pub id: String,
    #[serde(rename = "fund_id")]
    pub fund_id: String,
    pub created: String,
    pub modified: String,
    #[serde(rename = "nav_ccq")]
    pub nav_ccq: String,
    #[serde(rename = "trade_date")]
    pub trade_date: String,
    #[serde(rename = "nav_change")]
    pub nav_change: String,
    #[serde(rename = "per_nav_change")]
    pub per_nav_change: String,
    #[serde(rename = "total_nav")]
    pub total_nav: String,
    #[serde(rename = "highest_level")]
    pub highest_level: String,
    #[serde(rename = "lowest_level")]
    pub lowest_level: String,
    #[serde(rename = "nav_date")]
    pub nav_date: String,
    #[serde(rename = "last_year_nav_ccq")]
    pub last_year_nav_ccq: String,
}

pub async fn _get_dragoncapital_nav(
    fund: &str,
) -> Result<(String, f32), Box<dyn std::error::Error>> {
    let url = format!(
        "{}?trade_code={}",
        DRAGONCAPITAL_BASE_URL,
        fund.to_uppercase()
    );

    let resp = reqwest::get(url).await?.json::<DragonCapitalNAV>().await?;

    Ok((resp.trade_code, resp.nav_ccq.parse::<f32>().unwrap()))
}

//----------  VinaCapital ----------//
pub async fn _get_vinacapital_nav(fund: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("action", String::from("getchartfundnav"));
    params.insert("fundname", fund.to_uppercase());

    let client = reqwest::Client::new();
    let resp = client
        .post(VINACAPITAL_BASE_URL)
        .form(&params)
        .send()
        .await?;

    let text = resp.text().await?;

    let fragment = Html::parse_fragment(&text);

    let selector = Selector::parse(r#".rpfundnavcontent"#).unwrap();

    // Response vec [current, highest, lowest]
    let mut nav_values: Vec<f32> = Vec::new();

    for element in fragment.select(&selector) {
        // Parse text
        if let Some(text) = element.text().next() {
            // Transform: 25,123.45 -> 25123.45
            let raw_text = text.trim().replace(',', "");
            // println!("{}", raw_text);
            nav_values.push(raw_text.parse::<f32>().unwrap());
        }
    }

    Ok(nav_values)
}
