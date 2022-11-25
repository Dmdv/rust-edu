mod errors;

use errors::{TransactionError, get_first_transaction_for, get_transactions};

fn main() -> Result<(), TransactionError> {
    let tss = get_transactions("trans/transactions.json")?;
    for t in tss {
        println!("{:?}", t);
    }

    let t =
        get_first_transaction_for("trans/transactions.json", "Dima")
            .ok_or("Couldn't find transaction for Dima")?;

    println!("First transaction for Dima is {:?}", t);

    Ok(())
}