mod location;
mod transaction;
mod utils;

use location::Continent;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use transaction::Transaction;
use utils::get_transactions_by_continent;

// Replace this depending on your relative file location
const FILE_PATH: &str = "./transactions.csv";

/**
 * Main function
 * Reads a CSV file and parses it into a vector of transactions
 */
fn main() {
    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines = Vec::new();

    // Groups "total" investment per continent
    let mut investment_by_continent: HashMap<String, f64> = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        // Skip title
        if idx == 0 {
            continue;
        }

        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        match parsed_transaction {
            Ok(tx) => {
                let continent = tx.continent.as_str().to_string();

                // If contains investment amount, update balance
                if investment_by_continent.contains_key(&continent) {
                    let prev_investment_amount = investment_by_continent[&continent];
                    investment_by_continent.insert(continent, prev_investment_amount + tx.amount);
                } else {
                    investment_by_continent.insert(continent, tx.amount);
                }
                transactions.push(tx)
            }
            Err(_) => skipped_lines.push((idx, line_str)),
        }
    }

    // Print out all valid transactions
    for tx in transactions.iter() {
        println!("{:?}", *tx);
    }

    // Print out total investment per continent
    for (continent, amount) in investment_by_continent.iter() {
        println!("Continent: '{}', Invested Amount: {}", *continent, *amount);
    }

    // Print out all transactions in Europe
    println!("Transactions in Europe");
    for tx in get_transactions_by_continent(&transactions, &Continent::Europe) {
        println!("{:?}", *tx);
    }

    // Print out all invalid transaction, and their corresponding lines
    for skipped_tx in skipped_lines.iter() {
        println!(
            "Line {} was skipped, string is as follows: '{}'",
            skipped_tx.0, skipped_tx.1
        );
    }
}
