use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asset {
    pub title: String,
    pub asset_type: AssetType,
    pub avg_buy_price: f64,
    pub amount: f64,
    pub current_price: f64,
    pub currency: Currency,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AssetType {
    Gold,
    MutualFund,
    VNStock,
    Crypto,
    NFT,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Currency {
    VND,
    USD,
    ETH,
    BTC,
    SOL,
}

impl Asset {
    // Constructor
    pub fn new(
        title: &str,
        asset_type: AssetType,
        initial_buy_price: f64,
        amount: f64,
        currency: Currency,
    ) -> Self {
        Self {
            title: String::from(title),
            asset_type,
            amount,
            avg_buy_price: initial_buy_price,
            current_price: initial_buy_price,
            currency,
        }
    }

    /// Calculate the asset's Return on Investment (ROI)
    pub fn asset_roi(self) -> f64 {
        ((self.current_price - self.avg_buy_price) / self.avg_buy_price) * 100.0
    }
}
