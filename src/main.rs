mod cash_flow;
mod defaults;
mod portfolio;
mod price_services;
mod provider;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{AddAssign, SubAssign};

use anyhow::Result;
use num::Zero;
use price_services::*;
use provider::types::DataProvider;

use crate::provider::alchemy::AlchemyDataProvider;
use crate::provider::dragon_capital::DragonCapitalDataProvider;
use crate::provider::sjc::SjcDataProvider;
use crate::provider::vina_capital::VinaCapitalDataProvider;

trait FungibleAsset {
    type AssetName: ToString + Debug;
    type Currency: Zero + AddAssign + SubAssign + From<f32>;
}

struct Stock {}

impl FungibleAsset for Stock {
    type AssetName = String;
    type Currency = f32;
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
            .fetch_asset_prices(Some(asset_name.to_string()))
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
    println!("{vesaf_funds:#?}",);

    println!("-----E1VFVN30 Price-----");
    let e1vfvn30_funds = Investment::<Stock, DragonCapitalDataProvider>::new()
        .fetch_asset_price("e1vfvn30".to_string())
        .await
        .unwrap();
    println!("{e1vfvn30_funds:#?}",);

    println!("-----Gold Price-----");
    let gold = Investment::<Gold, SjcDataProvider>::new()
        .fetch_asset_price("e1vfvn30".to_string())
        .await
        .unwrap();
    println!("{gold:#?}");

    println!("-----NFT Price-----");
    let nakamigos = Investment::<NFT, AlchemyDataProvider>::new()
        .fetch_asset_price("nakamigos".to_string())
        .await
        .unwrap();
    println!("{nakamigos:#?}");

    println!("-----Crypto Price-----");
    let token_symbols = vec![
        "BTCUSDT".to_string(),
        "ETHUSDT".to_string(),
        "C98USDT".to_string(),
    ];
    let token_tickers = crypto::_get_ticker_change(token_symbols).await.unwrap();
    println!("{token_tickers:#?}]");
    // let market_prices = crypto::_get_coingecko_market().await;
    // match market_prices {
    //     Ok(res) => println!("{res:#?}"),
    //     Err(e) => println!("{e:#?}"),
    // }
    println!("-----VNStock Price-----");
    // Get today's date
    // let today = Local::now();

    // Format date as YYYY-MM-DD
    // let today_str = format!(
    //     "{:04}-{:02}-{:02}",
    //     today.year(),
    //     today.month(),
    //     today.day()
    // );
    // let stock_symbols = vec!["TCB".to_string(), "VCB".to_string(), "FPT".to_string()];
    // let stock_tickers =
    //     vn_stock::_get_ticker_change(stock_symbols, VNStockPlatform::VNDIRECT(today_str, None))
    //         .await;
    // match stock_tickers {
    //     Ok(res) => println!("{res:#?}"),
    //     Err(e) => println!("{e:#?}"),
    // }
    // println!("-----NFT Floor Price-----");
    // let nakamigos_price =
    //     nft::_get_nft_floor_price("0xd774557b647330C91Bf44cfEAB205095f7E6c367".to_string())
    //         .await
    //         .unwrap();
    // println!("{nakamigos_price:#?}");
}
