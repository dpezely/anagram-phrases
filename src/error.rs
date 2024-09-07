use thiserror::Error;
use std::convert::From;

pub type Result<T> = std::result::Result<T, AnagramError>;

#[derive(Debug, Error, PartialEq)]
#[must_use]
pub enum AnagramError {
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


impl From<std::io::Error> for AnagramError {
    fn from(err: std::io::Error) -> AnagramError {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File or directory path not found: {:?}", err);
                AnagramError::NoFilePath
            }
            _ => {
                println!("IO Error: {:?}", err);
                AnagramError::UnknownIoError
            }
        }
    }
}

impl From<std::io::ErrorKind> for AnagramError {
    fn from(err: std::io::ErrorKind) -> AnagramError {
        match err {
            std::io::ErrorKind::NotFound => {
                println!("File or directory path not found: {:?}", err);
                AnagramError::NoFilePath
            }
            _ => {
                println!("IO Error: {:?}", err);
                AnagramError::UnknownIoError
            }
        }
    }
}
