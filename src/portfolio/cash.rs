use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct TermDeposit {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub initial_investment: f64,
    pub interest_rate: f64,
}

/// Function to calculate total money return after a period with yearly interest rate
fn calculate_saving_return(
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
