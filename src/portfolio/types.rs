use crate::provider::types::DataProvider;
use anyhow::Result;
use num::Zero;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{AddAssign, SubAssign};

pub trait FungibleAsset {
    type Id: From<i64>;
    type Symbol: ToString + Debug;
    type Name: ToString + Debug;
    type Currency;
    type CurrentPrice: Zero + AddAssign + SubAssign + From<f32>;
    type CreatedAt: ToString + Debug;
    type Trades;
}

pub trait AssetManager<T: FungibleAsset> {
    async fn fetch_asset_price(&self, asset_name: T::Symbol) -> Result<T::CurrentPrice>;
    // TODO: Implement calc_roi(), getter() methods
}

pub struct Investment<T, D>
where
    T: FungibleAsset,
{
    _provider: PhantomData<D>,
    _type: PhantomData<T>,
}

impl<T, D> Investment<T, D>
where
    T: FungibleAsset,
    D: DataProvider,
{
    pub fn new() -> Self {
        return Self {
            _provider: PhantomData::default(),
            _type: PhantomData::default(),
        };
    }
}

impl<A, D> AssetManager<A> for Investment<A, D>
where
    A: FungibleAsset,
    D: DataProvider,
{
    async fn fetch_asset_price(&self, asset_name: A::Symbol) -> Result<A::CurrentPrice> {
        let provider = D::new();
        let output = provider
            .fetch_asset_price(Some(asset_name.to_string()))
            .await
            .unwrap();
        Ok(output.into())
    }
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: i64,
    pub portfolio_id: i64,
    pub action: TradeAction,
    pub amount: f64,
    pub price: f64,
    pub created_at: String, // Datetime
}

#[derive(Debug, Clone)]
pub enum TradeAction {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub enum Currency {
    VND,
    USD,
    ETH,
    BTC,
    SOL,
}
