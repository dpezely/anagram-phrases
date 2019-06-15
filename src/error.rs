use failure::Fail;
use std::convert::From;


#[derive(Debug, Fail, PartialEq)]
#[must_use]
pub enum ErrorKind {
    #[fail(display="Character is outside of expected character set range")]
    CharOutOfBounds,

    #[fail(display="Reject words with chars beyond that of input")]
    MismatchedChars,
    
    #[fail(display="File not found")]
    NoFilePath,

    #[fail(display="Prime number is larger than Big Num library can accommodate")]
    PrimeTooBig,
    
    #[fail(display="Unknown IO error")]
    UnknownIoError,

    #[fail(display="Product of Primes for word not a factor of input")]
    WordProductNotFactor,

    #[fail(display="Product of Primes for word is larger than that of input")]
    WordProductTooBig,

    #[fail(display="Reject words longer than input pattern")]
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
