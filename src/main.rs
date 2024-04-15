mod cash_flow;
mod defaults;
mod market;
mod portfolio;
mod provider;

// use crate::provider::alchemy::AlchemyDataProvider;
use crate::provider::binance::BinanceDataProvider;
use crate::provider::dragon_capital::DragonCapitalDataProvider;
use crate::provider::sjc::SjcDataProvider;
use crate::provider::tcbs::TCBSDataProvider;
use crate::provider::vina_capital::VinaCapitalDataProvider;
use crate::provider::vndirect::VNDirectDataProvider;

use crate::portfolio::crypto::Crypto;
use crate::portfolio::gold::Gold;
use crate::portfolio::stock::Stock;
use crate::portfolio::types::*;

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

    // println!("-----NFT Price-----");
    // let nakamigos = Investment::<NFT, AlchemyDataProvider>::new()
    //     .fetch_asset_price("nakamigos".to_string())
    //     .await
    //     .unwrap();
    // println!("Nakamigos: {nakamigos:#?}");

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
