use serde_derive::*;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl From<Error> for TransactionError {
    fn from(e: Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
}

#[derive(Debug)]
pub enum TransactionError {
    LoadError(Error),
    ParseError(serde_json::Error),
    Mess(&'static str)
}

#[allow(unused)]
pub fn get_transaction_a(fname: &str) -> Result<Vec<Transaction>, String> {
    // this is equivalent to `get_transaction_c`
    Ok(
        match serde_json::from_str(
            &match read_to_string(fname) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            }) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        },
    )
}

#[allow(unused)]
pub fn get_transaction_b(fname: &str) -> Result<Vec<Transaction>, String> {
    read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // ? marks works because TransactionError implements From
    Ok(serde_json::from_str(&read_to_string(fname)?)?)
}

// Options
pub fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
    let ts = get_transactions(fname).ok()?;
    for t in ts {
        if t.from == uname {
            return Some(t);
        }
    }

    None
}