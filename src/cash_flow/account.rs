use crate::cash_flow::transaction::*;

pub struct Account {
    name: String,
    balance: f64,
    transactions: Vec<Transaction>,
}
impl Account {
    // Constructor
    pub fn new(name: &str, initial_balance: f64) -> Self {
        Self {
            name: String::from(name),
            balance: initial_balance,
            transactions: Vec::new(),
        }
    }

    // Method to deposit funds into the account
    pub fn deposit(&mut self, amount: f64, description: &str, category: TransactionCategory) {
        self.balance += amount;
        let transaction = Transaction::new(amount, description, category);
        self.transactions.push(transaction);
    }

    // Method to withdraw funds from the account
    pub fn withdraw(
        &mut self,
        amount: f64,
        description: &str,
        category: TransactionCategory,
    ) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            let transaction = Transaction::new(-amount, description, category);
            self.transactions.push(transaction);
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }

    // Method to transfer funds to another account
    pub fn transfer(
        &mut self,
        amount: f64,
        description: &str,
        category: TransactionCategory,
        recipient: &mut Account,
    ) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            let transaction = Transaction::new(-amount, description, category.clone());
            self.transactions.push(transaction);

            recipient.balance += amount;
            let transaction = Transaction::new(amount, description, category);
            recipient.transactions.push(transaction);

            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }
}
