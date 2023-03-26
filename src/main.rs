mod location;
mod transaction;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use transaction::Transaction;

fn main() {
    let file = File::open("./transactions.csv").unwrap();
    let reader = BufReader::new(file);

    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines = Vec::new();
    let mut investment_to_continent: HashMap<String, f64> = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }

        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        match parsed_transaction {
            Ok(x) => {
                let continent = x.continent.as_str().to_string();

                // If contains investment amount, update balance
                if investment_to_continent.contains_key(&continent) {
                    let prev_investment_amount = investment_to_continent[&continent];
                    investment_to_continent.insert(continent, prev_investment_amount + x.amount);
                } else {
                    investment_to_continent.insert(continent, x.amount);
                }
                transactions.push(x)
            }
            Err(_) => skipped_lines.push((idx, line_str)),
        }
    }

    for tx in transactions.iter() {
        println!("{:?}", *tx);
    }

    for (continent, amount) in investment_to_continent.iter() {
        println!("Continent: '{}', Invested Amount: {}", *continent, *amount);
    }

    /*
    BONUS:
       Utilize HashMap to keep track of the total invested amount per continent
           and print the result out for each continent
           - Hint: You would need to convert the continent to String to store as keys
           - Create a function that takes in a reference slice of transactions and a
           reference of Continent, and filters rows by the Continent. Print only
           transactions with European companies
           - Hint: You would need to utilise lifetimes, iterators, and filter functio
    */

    for skipped_tx in skipped_lines.iter() {
        println!(
            "Line {} was skipped, string is as follows: '{}'",
            skipped_tx.0, skipped_tx.1
        );
    }
}
