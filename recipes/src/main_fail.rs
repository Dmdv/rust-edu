extern crate transactions;

use transactions::*;
use failure::Error;

fn main() -> Result<(), Error> {
    let tss = get_transactions("data/transactions_failed.json")?;
    for t in tss {
        println!("{:?}", t);
    }

    let t =
        get_first_transaction_for("data/transactions.json", "Steve")
            .ok_or(TransactionError::Mess("Couldn't find transaction for Dima"))?;

    println!("First transaction for Steve is {:?}", t);

    Ok(())
}