use thiserror::Error;

pub type Result<T> = std::result::Result<T, AnagramError>;

#[derive(Debug, Error)]
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

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Product of Primes for word not a factor of input")]
    WordProductNotFactor,

    #[error("Product of Primes for word is larger than that of input")]
    WordProductTooBig,

    #[error("Reject words longer than input pattern")]
    WordTooLong,

    #[error("The requested language is not implemented")]
    LangNotImplemented,
}
