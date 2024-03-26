use std::collections::HashMap;

use anyhow::Result;
use scraper::{Html, Selector};

use crate::defaults::VINACAPITAL_BASE_URL;

use super::DataProvider;

pub struct VinaCapitalDataProvider {
    base_url: String,
}

impl DataProvider for VinaCapitalDataProvider {
    fn new() -> Self {
        return VinaCapitalDataProvider {
            base_url: VINACAPITAL_BASE_URL.to_string(),
        };
    }

    async fn fetch_asset_prices(&self, fund: Option<String>) -> Result<f32> {
        let mut params = HashMap::new();
        params.insert("action", String::from("getchartfundnav"));
        params.insert("fundname", fund.unwrap().to_uppercase());
        let client = reqwest::Client::new();
        let resp = client
            .post(self.base_url.clone())
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
        Ok(nav_values.first().cloned().unwrap_or_default())
    }
}
