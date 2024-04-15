use super::{Currency, FungibleAsset, Trade};

pub struct Gold {}

impl FungibleAsset for Gold {
    type Id = i64;
    type Symbol = String;
    type Name = String;
    type Currency = Currency;
    type CurrentPrice = f64;
    type CreatedAt = String;
    type Trades = Vec<Trade>;
}
