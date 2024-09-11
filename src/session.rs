use num_bigint::BigUint;
use std::path::PathBuf;

use crate::error::Result;
use crate::languages::{Encoding, Language};
use crate::primes;

/// First few fields are identical to those of `Options`.
pub struct Session<'a> {
    // Same as from `Options` struct in src/bin/anagrams.rs:
    pub lang: Language,
    pub dict_file_paths: &'a [PathBuf],
    pub max_phrase_words: usize,
    pub skip_upcase: bool,
    pub skip_short: bool,
    pub encoding: Encoding,
    pub verbose: bool,
    pub input_string: String,

    // Computed values:
    pub input_phrase: &'a [String],
    pub pattern: String,
    pub essential: String,
    pub primes: Vec<u16>,
    pub primes_product: BigUint,
}

impl<'a> Session<'a> {
    /// Creates new `Session` instance.  Ostensibly, this is a
    /// constructor but returning `Result` around `Session`.
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        lang: &'a Language, dict_file_paths: &'a [PathBuf], encoding: Encoding,
        max_phrase_words: usize, skip_upcase: bool, skip_short: bool, verbose: bool,
        input_phrase: &'a [String],
    ) -> Result<Session<'a>> {
        let max_phrase_words = if max_phrase_words == 0 && input_phrase.len() >= 2 {
            input_phrase.len()
        } else if max_phrase_words < 2 {
            2
        } else {
            max_phrase_words
        };
        let input_string = input_phrase.join(" ");
        let pattern = primes::extract_unique_chars(&input_string);
        let essential = primes::essential_chars(&input_string);
        let primes = primes::primes(&essential)?;
        let primes_product = primes::primes_product(&primes)?;
        Ok(Session {
            lang: lang.clone(),
            dict_file_paths,
            max_phrase_words,
            encoding,
            skip_upcase,
            skip_short,
            verbose,
            input_string: input_string.to_string(),
            input_phrase,
            pattern,
            essential,
            primes,
            primes_product,
        })
    }
}
