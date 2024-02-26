mod crypto;
mod gold;
mod mutual_funds;
mod vn_stock;

#[tokio::main]
async fn main() {
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

    println!("-----VNStock Price-----");
    let stock_symbols = vec!["TCB".to_string(), "VCB".to_string(), "FPT".to_string()];
    let stock_tickers = vn_stock::_get_ticker_change(stock_symbols).await.unwrap();
    println!("{stock_tickers:#?}");
}
