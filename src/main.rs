mod cash_flow;
mod defaults;
mod market;
mod portfolio;
mod provider;

use crate::provider::alchemy::AlchemyDataProvider;
use crate::provider::binance::BinanceDataProvider;
use crate::provider::dragon_capital::DragonCapitalDataProvider;
use crate::provider::sjc::SjcDataProvider;
use crate::provider::tcbs::TCBSDataProvider;
use crate::provider::vina_capital::VinaCapitalDataProvider;
use crate::provider::vndirect::VNDirectDataProvider;
use anyhow::Result;
use num::Zero;
use provider::types::DataProvider;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{AddAssign, SubAssign};

trait FungibleAsset {
    type AssetName: ToString + Debug;
    type Currency: Zero + AddAssign + SubAssign + From<f32>;
}

trait AssetManager<T: FungibleAsset> {
    async fn fetch_asset_price(&self, asset_name: T::AssetName) -> Result<T::Currency>;
}

struct Investment<T, D>
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
    async fn fetch_asset_price(&self, asset_name: A::AssetName) -> Result<A::Currency> {
        let provider = D::new();
        let output = provider
            .fetch_asset_price(Some(asset_name.to_string()))
            .await
            .unwrap();
        Ok(output.into())
    }
}

struct Gold {}

impl FungibleAsset for Gold {
    type AssetName = String;
    type Currency = f32;
}

struct Stock {}

impl FungibleAsset for Stock {
    type AssetName = String;
    type Currency = f32;
}

struct Crypto {}

impl FungibleAsset for Crypto {
    type AssetName = String;
    type Currency = f32;
}

struct NFT {}

impl FungibleAsset for NFT {
    // Yes I know it's dumb af
    type AssetName = String;
    type Currency = f32;
}

#[tokio::main]
async fn main() {
    println!("-----VESAF Price-----");
    let vesaf_funds = Investment::<Stock, VinaCapitalDataProvider>::new()
        .fetch_asset_price("vesaf".to_string())
        .await
        .unwrap();
    println!("VESAF: {vesaf_funds:#?}",);

    println!("-----E1VFVN30 Price-----");
    let e1vfvn30_funds = Investment::<Stock, DragonCapitalDataProvider>::new()
        .fetch_asset_price("e1vfvn30".to_string())
        .await
        .unwrap();
    println!("E1VFVN30: {e1vfvn30_funds:#?}",);

    println!("-----Gold Price-----");
    let gold = Investment::<Gold, SjcDataProvider>::new()
        .fetch_asset_price("e1vfvn30".to_string())
        .await
        .unwrap();
    println!("SJC Gold: {gold:#?}");

    println!("-----NFT Price-----");
    let nakamigos = Investment::<NFT, AlchemyDataProvider>::new()
        .fetch_asset_price("nakamigos".to_string())
        .await
        .unwrap();
    println!("Nakamigos: {nakamigos:#?}");

    println!("-----Crypto Price-----");
    let eth = Investment::<Crypto, BinanceDataProvider>::new()
        .fetch_asset_price("eth".to_string())
        .await
        .unwrap();
    println!("ETH: {eth:#?}");

    println!("-----VNStock Price-----");
    let vcb_tcbs = Investment::<Stock, TCBSDataProvider>::new()
        .fetch_asset_price("vcb".to_string())
        .await
        .unwrap();
    println!("VCB (TCBS): {vcb_tcbs:#?}");

    let vcb_vndirect = Investment::<Stock, VNDirectDataProvider>::new()
        .fetch_asset_price("vcb".to_string())
        .await
        .unwrap();
    println!("VCB (VNDirect): {vcb_vndirect:#?}");
}
