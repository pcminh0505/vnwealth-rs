mod crypto;
mod gold;
mod mutual_funds;
mod portfolio;
mod vn_stock;

use chrono::{Datelike, Local};
use portfolio::*;
use vn_stock::VNStockPlatform;

#[tokio::main]
async fn main() {
    let portfolio = Portfolio {
        name: String::from("My Portfolio"),
        balance: 10000.0,
        investment: vec![
            Investment::Cash(Cash::EmergencyCash(5000.0)),
            Investment::Cash(Cash::TermDeposit(TermDepositDetail {
                title: String::from("Travel Saving"),
                start_date: String::from("2024-02-26"),
                end_date: String::from("2024-08-26"),
                interest_rate: 0.05,
                initial_investment: 3000.0,
            })),
            Investment::Asset(Asset::Gold(AssetDetail {
                avg_buy_price: 50.0,
                amount: 10.0,
                current_price: 60.0,
                title: String::from("Gold"),
            })),
            Investment::Asset(Asset::Stock(AssetDetail {
                avg_buy_price: 100.0,
                amount: 20.0,
                current_price: 110.0,
                title: String::from("Stock"),
            })),
        ],
    };

    let (total_cash_balance, total_asset_balance, balance_by_asset) =
        portfolio.total_balance_by_investment_type();
    println!("Total Cash Balance: {}", total_cash_balance);
    println!("Total Asset Balance: {}", total_asset_balance);
    println!("Balance by Asset: {:#?}", balance_by_asset);

    for asset in &portfolio.investment {
        match asset {
            Investment::Asset(asset) => match asset {
                Asset::Gold(detail)
                | Asset::Stock(detail)
                | Asset::Crypto(detail)
                | Asset::MutualFund(detail) => {
                    let asset_roi = portfolio.asset_roi(detail.avg_buy_price, detail.current_price);
                    println!("{}: ROI: {:.2}%", detail.title, asset_roi);
                }
            },
            _ => {}
        }
    }
    // println!("-----VESAF Funds-----");
    // let vinacap_nav = mutual_funds::_get_vinacapital_nav("vesaf").await.unwrap();
    // println!("{vinacap_nav:#?}",);

    // println!("-----E1VFVN30 Funds-----");
    // let dragoncap_nav = mutual_funds::_get_dragoncapital_nav("e1vfvn30")
    //     .await
    //     .unwrap();
    // println!("{dragoncap_nav:#?}",);

    // println!("-----Gold Price-----");
    // let gold_price = gold::_get_vn_gold_price().await.unwrap();
    // println!("{gold_price:#?}");

    // println!("-----Crypto Price-----");
    // let token_symbols = vec![
    //     "BTCUSDT".to_string(),
    //     "ETHUSDT".to_string(),
    //     "C98USDT".to_string(),
    // ];
    // let token_tickers = crypto::_get_ticker_change(token_symbols).await.unwrap();
    // println!("{token_tickers:#?}]");
    // let market_prices = crypto::_get_coingecko_market().await;
    // match market_prices {
    //     Ok(res) => println!("{res:#?}"),
    //     Err(e) => println!("{e:#?}"),
    // }
    // println!("-----VNStock Price-----");
    // // Get today's date
    // let today = Local::now();

    // // Format date as YYYY-MM-DD
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
}
