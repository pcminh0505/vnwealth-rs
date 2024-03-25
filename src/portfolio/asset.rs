use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub title: String,
    pub asset_type: AssetType,
    pub avg_buy_price: f64,
    pub amount: f64,
    pub current_price: f64,
    pub currency: Currency,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AssetType {
    Gold,
    MutualFund,
    VNStock,
    Crypto,
    NFT,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Currency {
    VND,
    USD,
    ETH,
    BTC,
    SOL,
}

/// Calculate the asset's Return on Investment (ROI)
pub fn asset_roi(avg_buy_price: f64, current_price: f64) -> f64 {
    ((current_price - avg_buy_price) / avg_buy_price) * 100.0
}
