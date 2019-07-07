use num_bigint::BigUint;

use crate::error::ErrorKind;
use crate::languages::Language;
use crate::primes;

/// First few fields are identical to those of `Options`.
pub struct Session<'a> {
    // Same as from `Options` struct in src/bin/anagrams.rs:
    pub lang: Language,
    pub dict_file_paths: Vec<String>,
    pub max_phrase_words: usize,
    pub skip_upcase: bool,
    pub skip_short: bool,
    pub iso_8859_1: bool,
    pub verbose: bool,
    pub input_string: String,

    // Computed values:
    pub input_phrase: Vec<&'a str>,
    pub pattern: String,
    pub essential: String,
    pub primes: Vec<u16>,
    pub primes_product: BigUint,
}

impl<'a> Session<'a> {
    /// Creates new `Session` instance.  Ostensibly, this is a
    /// constructor but returning `Result` around `Session`.
    #[allow(clippy::too_many_arguments)]
    pub fn start(lang: &'a Language, dict_file_paths: Vec<String>,
                 iso_8859_1: bool, max_phrase_words: usize,
                 skip_upcase: bool, skip_short: bool, verbose: bool,
                 input_string: &'a str) -> Result<Session<'a>, ErrorKind> {
        let mut dict_file_paths = dict_file_paths.clone();
        let mut max_phrase_words = max_phrase_words;
        if dict_file_paths.is_empty() {
            dict_file_paths.push("/usr/share/dict/words".to_string());
        }
        if max_phrase_words == 0 {
            let n = input_string.trim()
                .chars()
                .fold(0, |acc,ch| acc + ch.is_whitespace() as usize);
            max_phrase_words = n;
        }
        if max_phrase_words < 2 {
            max_phrase_words = 2;
        }
        let input_phrase: Vec<&'a str> = input_string.split_whitespace().collect();
        let pattern = primes::extract_unique_chars(&input_string);
        let essential = primes::essential_chars(&input_string);
        let primes = primes::primes(&essential)?;
        let primes_product = primes::primes_product(&primes)?;
        Ok(Session{lang: lang.clone(), dict_file_paths, max_phrase_words,
                   iso_8859_1, skip_upcase, skip_short, verbose,
                   input_string: input_string.to_string(), input_phrase,
                   pattern, essential, primes, primes_product})
    }
}
