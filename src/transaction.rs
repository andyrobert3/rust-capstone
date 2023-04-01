use crate::location::{Continent, Country};
use chrono::NaiveDate;

const NUM_FIELDS_PER_LINE: usize = 7;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Transaction {
    pub continent: Continent,
    pub amount: f64,
    pub transaction_id: u32,
    client_id: u32,
    asset_name: String,
    country: Country,
    days_under_management: i64,
}

impl Transaction {
    /**
     * Parses a line from the CSV file into a Transaction struct as defined above
     * Throws an error if the line is not in the correct format
     */
    pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() != NUM_FIELDS_PER_LINE {
            return Err("Incorrect number of columns".to_string());
        }

        let transaction_id: u32 = fields[0].parse::<u32>().unwrap();
        let client_id: u32 = fields[1].parse::<u32>().unwrap();
        let asset_name: String = fields[2].to_uppercase();
        let transaction_start_date = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
        let transaction_end_date = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();

        let country = fields[5].parse::<Country>()?;
        let amount: f64 = fields[6].parse::<f64>().unwrap();

        let days_under_management = (transaction_end_date - transaction_start_date).num_days();
        if days_under_management < 0 {
            return Err(format!(
                "Transaction start date: {} is later than end date: {}",
                transaction_start_date.to_string(),
                transaction_end_date.to_string()
            ));
        }

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
