use crate::location::{Continent, Country};
use chrono::NaiveDate;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Transaction {
    transaction_id: u32,
    client_id: u32,
    asset_name: String,
    country: Country,
    continent: Continent,
    amount: f64,
    days_under_management: i64,
}

impl Transaction {
    pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() != 7 {
            return Err("Incorrect number of columns".to_string());
        }

        let transaction_id: u32 = fields[0].parse::<u32>().unwrap();
        let client_id: u32 = fields[1].parse::<u32>().unwrap();
        let asset_name: String = fields[2].to_uppercase();
        let transaction_start_date = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
        let transaction_end_date = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();

        // https://doc.rust-lang.org/std/str/trait.FromStr.html
        let country = fields[5].parse::<Country>()?;
        let amount: f64 = fields[6].parse::<f64>().unwrap();

        let days_under_management = (transaction_end_date - transaction_start_date).num_days();
        let continent = country.country_to_continent();

        let transaction = Transaction {
            asset_name,
            transaction_id,
            client_id,
            country,
            continent,
            amount,
            days_under_management,
        };

        Ok(transaction)
    }
}
