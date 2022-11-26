extern crate transactions;
use transactions::*;

fn main() -> Result<(), TransactionError> {
    let tss = get_transactions("data/transactions.json")?;
    for t in tss {
        println!("{:?}", t);
    }

    let t =
        get_first_transaction_for("data/transactions.json", "Steve")
            .ok_or("Couldn't find transaction for Dima")?;

    println!("First transaction for Steve is {:?}", t);

    Ok(())
}