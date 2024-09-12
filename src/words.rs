//! Load word lists with or without filtering.

use num_bigint::BigUint;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::config::Config;
use crate::error::Result;
use crate::languages::{self, Encoding, Language, SHORT, UPCASE};
use crate::primes::{self, PMap};

/// Cache of a word list and its metadata.
///
/// For a single use runtime such as the CLI version, probably there
/// will be only one instance of this struct.  However, in multi-language
/// runtimes such as the HTTP version, each $LANG gets its own.
pub struct Cache<'a> {
    /// HashMap of prime to phrase
    pub lexicon: &'a PMap,
    /// Index into `dictionary` sorted by its keys (primes) high-to-low
    pub descending_keys: Vec<&'a BigUint>,
}

impl<'a> Cache<'a> {
    /// Constructor for use after loading word list.
    /// Unnecessary when discovering only transpositions (single word
    /// to single word anagrams).
    /// Generates internal metadata necessary for subsequent searches.
    /// ```ignore
    /// let search = Search::query(&input_phrase, &config)?;
    /// let (dict, _singles) =
    ///   words::load_and_select(&config, &pattern, &essential, &primes_product)?;
    /// let cache = Cache::init(&dict);
    /// ```
    pub fn init(map: &PMap) -> Cache {
        let mut keys: Vec<&BigUint> = map.keys().collect();
        keys.sort_by(|a, b| b.cmp(a));

        // TODO: Apply modified sequence of primes to accommodate
        // letter frequency within locale specific $LANG, such that
        // more common words will be found first; e.g., ETAOIN SRHLDCU
        // in EN-US from https://norvig.com/mayzner.html

        Cache { lexicon: map, descending_keys: keys }
    }
}

const NEWLINE: u8 = 0x0A;

/// Filter while loading lists of natural language words: e.g., English.
///
/// Reject words based upon various criteria: 1) too long to possibly
/// match; 2) containing characters other than those from the input
/// pattern; 3) words with a product greater than that of the input
/// phrase.
///
/// Select and return list of words exactly matching primes product of
/// input phrase, and exclude those from the resulting [PMap].
///
/// Params:
/// - `pattern` Set of unique characters extracted from query;
/// - `essential` Alphabetic characters of query including duplicates;
/// - `primes_product` Mathematical product of all prime numbers representing `pattern`;
/// - `lang` and `encoding` Language (e.g., EN=English), UTF-8/ISO-8859-1/etc;
/// - `short` and `upcase` opt-in to allowing words that otherwise
///    aren't idiomatic for `lang` (i.e., Booleans to allow more than
///    'a' and 'I' for English.)
///
/// Returns tuple of 1) [PMap] containing words selected after initial
/// filtering and 2) set of single word matches.
///
/// See also: fn [preload_wordlist].
pub fn load_and_select(
    config: &Config, pattern: &str, essential: &str, primes_product: &BigUint,
) -> Result<(PMap, Vec<String>)> {
    let mut single_word_list = vec![];
    let mut map = PMap::new();
    let input_length = essential.len();
    let empty: Vec<&str> = vec![];
    let short_words = SHORT.get(&config.lang).unwrap_or(&empty);
    let upcase_words = UPCASE.get(&config.lang).unwrap_or(&empty);
    let mut bytes: Vec<u8> = vec![];
    for filepath in config.dict_file_paths.iter() {
        let mut fd = BufReader::new(File::open(filepath)?);
        let mut word = String::new();
        let mut previous = String::new();
        let mut i = 0;
        loop {
            i += 1;
            bytes.clear();
            word.clear();
            match fd.read_until(NEWLINE, &mut bytes) {
                Ok(0) => break, // End of file (EOF)
                Ok(_n) => {
                    if config.encoding == Encoding::Iso_8859_1 {
                        word = bytes.iter().map(|&x| char::from(x)).collect();
                    } else {
                        word = String::from_utf8_lossy(&bytes).to_string();
                    }
                    word = word.trim().to_string();
                    if word.is_empty() {
                        continue;
                    }
                    if word == previous {
                        continue;
                    }
                    if languages::filter(
                        &word,
                        short_words,
                        upcase_words,
                        config.include_short,
                        config.include_upcase,
                    ) {
                        continue;
                    }
                    if let Ok(product) =
                        primes::filter_word(&word, pattern, input_length, primes_product)
                    {
                        if product == *primes_product {
                            // This dictionary word matches exactly.
                            single_word_list.push(word.to_string());
                        } else {
                            // Store remaining words in look-up table:
                            map.entry(product)
                                .or_insert_with(|| Vec::with_capacity(1))
                                .push(word.to_string());
                        }
                    }
                    std::mem::swap(&mut previous, &mut word);
                }
                Err(e) => {
                    println!(
                        "File error: file={} line={i} {e:?}",
                        filepath.to_string_lossy()
                    );
                    break;
                }
            }
        }
        println!(
            "Word list: file={} lines={i}, filtered-entries={}",
            filepath.to_string_lossy(),
            map.len()
        );
    }
    Ok((map, single_word_list))
}

/// Load ENTIRE word list suitable for caching and WITHOUT filtering.
/// Intended to be invoked once per natural language and cached, which
/// is suitable for persistent HTTP service.
///
/// Params with same name have identical semantics as [load_and_select].
/// Note: consider caching with `short` ane `upcase` enabled, but
/// apply same filtering per query based upon user preferences.
///
/// Returns [PMap] containing mathematical product of primes
/// associated with list of words with that product.
///
/// See also: fn [load_and_select].
pub fn preload(
    file_paths: &[&Path], lang: &Language, encoding: &Encoding, short: bool,
    upcase: bool, verbose: bool,
) -> Result<PMap> {
    let mut map = PMap::new();
    let empty: Vec<&str> = vec![];
    let short_words = SHORT.get(lang).unwrap_or(&empty);
    let upcase_words = UPCASE.get(lang).unwrap_or(&empty);
    let mut bytes: Vec<u8> = vec![];
    for filepath in file_paths {
        let mut word = String::new();
        let mut previous = String::new();
        let mut i = 0;
        loop {
            i += 1;
            bytes.clear();
            word.clear();
            let mut f = BufReader::new(File::open(filepath)?);
            match f.read_until(0x0A, &mut bytes) {
                Ok(0) => break, // End of file (EOF)
                Ok(_n) => {
                    if *encoding == Encoding::Iso_8859_1 {
                        word = bytes.iter().map(|&x| char::from(x)).collect();
                    } else {
                        word = String::from_utf8_lossy(&bytes).to_string();
                    }
                    word = word.trim().to_string();
                    if word.is_empty() {
                        continue;
                    }
                    if word == previous {
                        continue;
                    }
                    if languages::filter(&word, short_words, upcase_words, short, upcase)
                    {
                        continue;
                    }
                    let essential = primes::essential_chars(&word);
                    let primes = primes::primes(&essential)?;
                    let product = primes::primes_product(&primes)?;
                    map.entry(product)
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(word.to_string());
                    std::mem::swap(&mut previous, &mut word);
                }
                Err(e) => {
                    println!(
                        "File error: file={} line={i} {e:?}",
                        filepath.to_string_lossy()
                    );
                    break;
                }
            }
        }
        if verbose {
            println!(
                "Word list: file={} lines={i}, filtered-entries={}",
                filepath.to_string_lossy(),
                map.len()
            );
        }
    }
    Ok(map)
}
