use std::io::Error;

#[derive(Debug)]
pub enum TransactionError {
    LoadError(Error),
    ParseError(serde_json::Error),
    Mess(&'static str)
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