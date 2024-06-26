// Define an enum to represent transaction categories
#[derive(Debug, Clone)]
pub enum TransactionCategory {
    Need,
    Want,
    Saving,
    Invest,
    Transfer,
}

// Define the Transaction struct
#[derive(Debug)]
pub struct Transaction {
    pub amount: f64,
    pub description: String,
    pub category: TransactionCategory,
}

// Implementation of Transaction methods
impl Transaction {
    // Constructor function for Transaction
    pub fn new(amount: f64, description: &str, category: TransactionCategory) -> Self {
        Self {
            amount,
            description: String::from(description),
            category,
        }
    }
}
