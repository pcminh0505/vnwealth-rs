use dotenv::dotenv;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

//---------- BASE URLs ----------//
const ALCHEMY_NFT_BASE_URL: &str = "https://eth-mainnet.g.alchemy.com/nft/v3";

//---------- Struct Definition ----------//
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlchemyNFTResponse {
    pub open_sea: OpenSea,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSea {
    pub floor_price: f64,
    pub price_currency: String,
    pub collection_url: String,
    pub retrieved_at: String,
    pub error: Value,
}

//---------- Functions ----------//
pub async fn _get_nft_floor_price(
    contract_address: String,
) -> Result<OpenSea, Box<dyn std::error::Error>> {
    dotenv().ok(); // Load the .env file

    // Read the value from the .env file
    let alchemy_api_key = env::var("ALCHEMY_API_KEY_ETH_MAINNET")
        .expect("You've not set the ALCHEMY_API_KEY_ETH_MAINNET");

    let query_params = format!("contractAddress={}", contract_address);

    let url = format!(
        r#"{}/{}/getFloorPrice?{}"#,
        ALCHEMY_NFT_BASE_URL, alchemy_api_key, query_params
    );

    // println!("{url:#?}");

    let resp = reqwest::get(url)
        .await?
        .json::<AlchemyNFTResponse>()
        .await?;
    // println!("{resp:#?}");

    Ok(resp.open_sea)
}
