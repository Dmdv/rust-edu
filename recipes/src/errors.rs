use std::io::Error;
use failure_derive::*;
// use thiserror::*;

#[derive(Debug,Fail)]
pub enum TransactionError {
    #[fail(display="Couldn't load file: {}", 0)]
    LoadError(Error),
    #[fail(display="Couldn't parse file: {}", 0)]
    ParseError(serde_json::Error),
    #[fail(display="Error: {}", 0)]
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