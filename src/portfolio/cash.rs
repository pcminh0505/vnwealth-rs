use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct TermDeposit {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub initial_amount: f64,
    pub interest_rate: f64,
}

impl TermDeposit {
    /// Constructor
    pub fn new(
        title: &str,
        start_date: &str,
        end_date: &str,
        initial_amount: f64,
        interest_rate: f64,
    ) -> Self {
        Self {
            title: String::from(title),
            start_date: String::from(start_date),
            end_date: String::from(end_date),
            initial_amount,
            interest_rate,
        }
    }

    /// Function to calculate total money return after a period with yearly interest rate
    pub fn calculate_saving_return(self) -> f64 {
        // Parse start and end dates from strings
        let start_date = NaiveDate::from_str(&self.start_date).expect("Invalid start date format");
        let end_date = NaiveDate::from_str(&self.end_date).expect("Invalid end date format");

        let percentage_change = ((end_date - start_date).num_days() as f64 / 30.0).floor()
            * (self.interest_rate / 12.0);

        let total_return = (self.initial_amount * (1.0 + percentage_change)).ceil();
        total_return
    }
}
