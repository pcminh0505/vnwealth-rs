mod errors;
mod pallets;
mod portfolio;
mod price_services;
mod support;

use chrono::{Datelike, Local};

use pallets::assets;
use pallets::assets::_fungible;
use pallets::system;
// use price_services::vn_stock::VNStockPlatform;
use price_services::*;

use crate::support::Dispatch;

pub struct StockAsset {
    system: system::Pallet<Self>,
    fungible: _fungible::Pallet<Self>,
}

pub enum ActionCall {
    Fungible(assets::_fungible::Call<StockAsset>),
}

impl pallets::system::Config for StockAsset {
    type AccountId = String;
}

impl assets::_fungible::Config for StockAsset {
    type AssetName = String;
    type Currency = u128;
}

impl StockAsset {
    pub fn new() -> Self {
        return Self {
            system: system::Pallet::new(),
            fungible: assets::_fungible::Pallet::new(),
        };
    }
}

impl crate::support::Dispatch for StockAsset {
    type Caller = <StockAsset as pallets::system::Config>::AccountId;
    type Call = ActionCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            ActionCall::Fungible(call) => self.fungible.dispatch(caller, call),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut stock_asset = StockAsset::new();

    let caller = "alice".to_string();
    stock_asset
        .dispatch(
            caller,
            ActionCall::Fungible(assets::_fungible::Call::Mint {
                asset_name: "CTG".to_string(),
                amount: 100,
            }),
        )
        .unwrap();

    println!("-----VESAF Funds-----");
    let vinacap_nav = mutual_funds::_get_vinacapital_nav("vesaf").await.unwrap();
    println!("{vinacap_nav:#?}",);

    println!("-----E1VFVN30 Funds-----");
    let dragoncap_nav = mutual_funds::_get_dragoncapital_nav("e1vfvn30")
        .await
        .unwrap();
    println!("{dragoncap_nav:#?}",);

    println!("-----Gold Price-----");
    let gold_price = gold::_get_vn_gold_price().await.unwrap();
    println!("{gold_price:#?}");

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
    let today = Local::now();

    // Format date as YYYY-MM-DD
    let today_str = format!(
        "{:04}-{:02}-{:02}",
        today.year(),
        today.month(),
        today.day()
    );
    let stock_symbols = vec!["TCB".to_string(), "VCB".to_string(), "FPT".to_string()];
    // let stock_tickers =
    //     vn_stock::_get_ticker_change(stock_symbols, VNStockPlatform::VNDIRECT(today_str, None))
    //         .await;
    // match stock_tickers {
    //     Ok(res) => println!("{res:#?}"),
    //     Err(e) => println!("{e:#?}"),
    // }

    println!("-----NFT Floor Price-----");
    let nakamigos_price =
        nft::_get_nft_floor_price("0xd774557b647330C91Bf44cfEAB205095f7E6c367".to_string())
            .await
            .unwrap();
    println!("{nakamigos_price:#?}");
}
