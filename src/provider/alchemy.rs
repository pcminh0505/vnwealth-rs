use super::DataProvider;
use crate::defaults::ALCHEMY_NFT_BASE_URL;
use anyhow::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

pub struct AlchemyDataProvider {
    base_url: String,
}

impl DataProvider for AlchemyDataProvider {
    fn new() -> Self {
        AlchemyDataProvider {
            base_url: ALCHEMY_NFT_BASE_URL.to_string(),
        }
    }

    async fn fetch_asset_price(&self, collection_slug: Option<String>) -> Result<f32> {
        dotenv().ok(); // Load the .env file

        // Read the value from the .env file
        let alchemy_api_key = env::var("ALCHEMY_API_KEY_ETH_MAINNET")
            .expect("You've not set the ALCHEMY_API_KEY_ETH_MAINNET");

        let query_params = format!("collectionSlug={}", collection_slug.unwrap());

        let url = format!(
            r#"{}/{}/getFloorPrice?{}"#,
            self.base_url.clone(),
            alchemy_api_key,
            query_params
        );

        let resp = reqwest::get(url)
            .await?
            .json::<AlchemyNFTResponse>()
            .await?;
        Ok(resp.open_sea.floor_price)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlchemyNFTResponse {
    pub open_sea: OpenSea,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSea {
    pub floor_price: f32,
    pub price_currency: String,
    pub collection_url: String,
    pub retrieved_at: String,
    pub error: Value,
}
