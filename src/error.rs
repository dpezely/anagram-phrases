use thiserror::Error;
use std::convert::From;

#[derive(Debug, Error, PartialEq)]
#[must_use]
pub enum ErrorKind {
    #[error("Character is outside of expected character set range")]
    CharOutOfBounds,

    #[error("Reject words with chars beyond that of input")]
    MismatchedChars,

    #[error("File not found")]
    NoFilePath,

    #[error("Prime number is larger than Big Num library can accommodate")]
    PrimeTooBig,

    #[error("Failed while attempting to export results")]
    SerializationError,

    #[error("Unknown IO error")]
    UnknownIoError,

    #[error("Product of Primes for word not a factor of input")]
    WordProductNotFactor,

    #[error("Product of Primes for word is larger than that of input")]
    WordProductTooBig,

    #[error("Reject words longer than input pattern")]
    WordTooLong,
}


impl From<std::io::Error> for ErrorKind {
    fn from(err: std::io::Error) -> ErrorKind {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File or directory path not found: {:?}", err);
                ErrorKind::NoFilePath
            }
            _ => {
                println!("IO Error: {:?}", err);
                ErrorKind::UnknownIoError
            }
        }
    }
}

impl From<std::io::ErrorKind> for ErrorKind {
    fn from(err: std::io::ErrorKind) -> ErrorKind {
        match err {
            std::io::ErrorKind::NotFound => {
                println!("File or directory path not found: {:?}", err);
                ErrorKind::NoFilePath
            }
            _ => {
                println!("IO Error: {:?}", err);
                ErrorKind::UnknownIoError
            }
        }
    }
}
