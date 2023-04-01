use crate::location::Continent;
use crate::transaction::Transaction;

/**
 * Returns a vector of transactions that are in the given continent
 *
 * # Arguments
 * - `transactions` - A vector of transactions
 * - `continent` - A continent
 *
 * # Returns
 * - A vector of transactions that are in the given continent
 */
pub fn get_transactions_by_continent<'a, 'b>(
    transactions: &'a [Transaction],
    continent: &'b Continent,
) -> Vec<&'a Transaction> {
    let continent_transactions = transactions
        .iter()
        .filter(|tx| tx.continent == *continent)
        .collect::<Vec<_>>();

    return continent_transactions;
}
