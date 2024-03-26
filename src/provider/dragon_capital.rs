use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::defaults::DRAGONCAPITAL_BASE_URL;

use super::DataProvider;

pub struct DragonCapitalDataProvider {
    base_url: String,
}

impl DataProvider for DragonCapitalDataProvider {
    fn new() -> Self {
        DragonCapitalDataProvider {
            base_url: DRAGONCAPITAL_BASE_URL.to_string(),
        }
    }

    async fn fetch_asset_prices(&self, fund: Option<String>) -> Result<f32> {
        let url = format!(
            "{}?trade_code={}",
            self.base_url.clone(),
            fund.unwrap().to_uppercase()
        );

        let resp = reqwest::get(url).await?.json::<DragonCapitalNAV>().await?;
        Ok(resp.nav_ccq.parse::<f32>().unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
