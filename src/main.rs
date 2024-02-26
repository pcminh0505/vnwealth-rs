mod gold;
mod mutual_funds;

#[tokio::main]
async fn main() {
    print!("-----VESAF Funds-----\n");
    let vinacap_nav = mutual_funds::_get_vinacapital_nav("vesaf").await.unwrap();
    print!(
        "{} NAV details:\nCurrent: {}\nHighest: {}\nLowest: {}\n",
        "VESAF", vinacap_nav[0], vinacap_nav[1], vinacap_nav[2]
    );

    print!("-----E1VFVN30 Funds-----\n");
    let dragoncap_nav = mutual_funds::_get_dragoncapital_nav("e1vfvn30")
        .await
        .unwrap();
    print!(
        "{} NAV details:\nCurrent: {}\n",
        dragoncap_nav.0, dragoncap_nav.1
    );
    print!("-----Gold Price-----\n");
    let gold_price = gold::_get_vn_gold_price().await.unwrap();
    print!("SJC Buy: {}\nSJC Sell: {}\n", gold_price.0, gold_price.1);
}
