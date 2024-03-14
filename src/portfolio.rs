use chrono::NaiveDate;
use std::collections::HashMap;
use std::str::FromStr;

// Define the Investment types
#[derive(Debug)]
pub enum Investment {
    Cash(Cash),
    Asset(Asset),
}

// Define the cash asset types
#[derive(Debug)]
pub enum Cash {
    EmergencyCash(f64), // Only care about the value
    TermDeposit(TermDepositDetail),
}

// Define the instrument asset types
#[derive(Debug)]
pub enum Asset {
    Gold(AssetDetail),
    Stock(AssetDetail),
    Crypto(AssetDetail),
    MutualFund(AssetDetail),
    // Add more instrument types as needed
}

// Define the Portfolio struct
#[derive(Debug)]
pub struct Portfolio {
    // pub id: i64,
    pub name: String,
    pub balance: f64,
    pub investment: Vec<Investment>,
    // pub created_at: i64,
    // pub updated_at: i64,
}

// Implementation of Portfolio methods
impl Portfolio {
    // Method to calculate the total balance for each asset group
    pub fn total_balance_by_investment_type(&self) -> (f64, f64, HashMap<String, f64>) {
        let mut total_cash_balance = 0.0;
        let mut total_asset_balance = 0.0;

        for item in &self.investment {
            match item {
                Investment::Cash(cash) => match cash {
                    Cash::EmergencyCash(value) => total_cash_balance += value,
                    Cash::TermDeposit(term) => {
                        total_cash_balance += calculate_return(
                            term.initial_investment,
                            term.interest_rate,
                            term.start_date.clone(),
                            term.end_date.clone(),
                        )
                    }
                },
                Investment::Asset(asset) => {
                    total_asset_balance += match asset {
                        Asset::Gold(detail) => detail.amount * detail.current_price,
                        Asset::Stock(detail) => detail.amount * detail.current_price,
                        Asset::Crypto(detail) => detail.amount * detail.current_price,
                        Asset::MutualFund(detail) => detail.amount * detail.current_price,
                    };
                }
            }
        }

        (
            total_cash_balance,
            total_asset_balance,
            self.balance_by_asset_type(),
        )
    }
    // Method to calculate the total balance for each sub-asset type
    pub fn balance_by_asset_type(&self) -> HashMap<String, f64> {
        let mut balances_by_asset: HashMap<String, f64> = HashMap::new();

        for asset in &self.investment {
            if let Investment::Asset(asset) = asset {
                let total_balance = match asset {
                    Asset::Gold(detail) => detail.amount * detail.current_price,
                    Asset::Stock(detail) => detail.amount * detail.current_price,
                    Asset::Crypto(detail) => detail.amount * detail.current_price,
                    Asset::MutualFund(detail) => detail.amount * detail.current_price,
                };

                let title = match asset {
                    Asset::Gold(detail) => &detail.title,
                    Asset::Stock(detail) => &detail.title,
                    Asset::Crypto(detail) => &detail.title,
                    Asset::MutualFund(detail) => &detail.title,
                };

                *balances_by_asset.entry(title.clone()).or_insert(0.0) += total_balance;
            } else {
            }
        }

        balances_by_asset
    }

    // Method to calculate the asset's Return on Investment (ROI)
    pub fn asset_roi(&self, avg_buy_price: f64, current_price: f64) -> f64 {
        ((current_price - avg_buy_price) / avg_buy_price) * 100.0
    }
}

#[derive(Debug)]
pub struct TermDepositDetail {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub initial_investment: f64,
    pub interest_rate: f64,
}

#[derive(Debug)]
pub struct AssetDetail {
    pub title: String,
    pub avg_buy_price: f64,
    pub amount: f64,
    pub current_price: f64,
}

// Function to calculate total money return after a period with yearly interest rate
fn calculate_return(
    initial_investment: f64,
    interest_rate: f64,
    start_date: String,
    end_date: String,
) -> f64 {
    // Parse start and end dates from strings
    let start_date = NaiveDate::from_str(&start_date).expect("Invalid start date format");
    let end_date = NaiveDate::from_str(&end_date).expect("Invalid end date format");

    let percentage_change =
        ((end_date - start_date).num_days() as f64 / 30.0).floor() * (interest_rate / 12.0);

    let total_return = (initial_investment * (1.0 + percentage_change)).ceil();
    total_return
}
