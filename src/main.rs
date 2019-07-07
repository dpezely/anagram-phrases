//! Command-line interface for solving anagrams and allows multiple
//! words for input and results.

extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate num_bigint;
extern crate num_traits;
extern crate structopt;

#[cfg(feature="external-hasher")]
extern crate char_seq;

use num_bigint::BigUint;
use std::collections::BTreeMap;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;
use structopt::StructOpt;

mod error;
mod languages;
mod primes;
mod search;
#[cfg(test)]
mod test_primes;

use crate::error::ErrorKind;
use crate::languages::{Available, SHORT, UPCASE};
use crate::primes::Map;

#[derive(StructOpt, Debug)]
#[structopt(max_term_width=80)]
struct Options {
    /// Specify 2 letter ISO code for natural language such as EN for
    /// English, FR for Français, etc. to enable specific filters.
    #[structopt(short="l", long="lang", required=false, default_value="any",
                raw(possible_values="&Available::variants()",
                    case_insensitive="true"))]
    lang: Available,

    /// Must be a plain-text file containing one word per line.
    /// Files suitable for `ispell` or GNU `aspell` are compatible.
    #[structopt(short="d", long="dict", multiple=true, number_of_values=1,
                default_value="/usr/share/dict/words")]
    dict_file_path: Vec<String>,

    /// Defaults to one more than number words of input phrase
    /// with a minimum of 3.
    #[structopt(short="m", long="max", default_value="0")]
    max_phrase_words: usize,

    /// Skip dictionary words containing uppercase, which indicates
    /// being a proper names.  However, use --lang=EN to allow "I" as
    /// an exception for English; --lang=ES allows "y" for Spanish; etc.
    #[structopt(short="u", long="upcase")]
    skip_upcase: bool,

    /// Skip dictionary words containing single letters, which may
    /// help avoid noisy results.  However, use --lang=en allowing
    /// only `a` and `I` for English, `y` for Spanish, etc.
    #[structopt(short="s", long="short")]
    skip_short: bool,

    /// Load dictionaries as ISO-8859-1 rather than UTF-8 encoding
    // FIXME: also convert from Latin-2, etc.
    #[structopt(short="1", long="iso-8859-1",
                raw(aliases = r#"&["alias"]"#,
                    possible_values = r#"&["latin-1", "latin1"]"#))]
    iso_8859_1: bool,

    /// Display additional status information
    #[structopt(short="v", long="verbose")]
    verbose: bool,

    /// Currently, only ASCII and ISO-8859-1 are supported.
    /// May be a single word or phrase consisting of multiple words.
    /// For a phrase, be sure to use quotes or escape spaces.
    #[structopt(name="PHRASE")]
    input_string: String,
}

/// First few fields are identical to those of `Options`.
struct Session<'a> {
    // Same as from `Options` struct:
    lang: Available,
    dict_file_path: Vec<String>,
    max_phrase_words: usize,
    skip_upcase: bool,
    skip_short: bool,
    iso_8859_1: bool,
    verbose: bool,
    input_string: String,

    // Computed values:
    input_phrase: Vec<&'a str>,
    pattern: String,
    essential: String,
    primes: Vec<u16>,
    primes_product: BigUint,
}

/// Resolve a single anagram phrase or word from command-line parameters.
fn main() -> Result<(), ErrorKind> {
    let opts = Options::from_args();
    let session = Session::start(&opts)?;
    resolve_single(&session)?;
    Ok(())
}

impl<'a> Session<'a> {
    /// Creates new `Session` instance, without modifying `options`.
    /// Ostensibly, this is a a constructor but returning `Result`
    /// around `Session`.
    fn start(options: &Options) -> Result<Session, ErrorKind> {
        let Options{lang, dict_file_path, iso_8859_1, max_phrase_words,
                    skip_upcase, skip_short, verbose, input_string} = options;
        let mut dict_file_path = dict_file_path.clone();
        let mut max_phrase_words = *max_phrase_words;
        if dict_file_path.is_empty() {
            dict_file_path.push("/usr/share/dict/words".to_string());
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
        let input_phrase: Vec<&str> = options.input_string.split(" ").collect();
        let pattern = primes::extract_unique_chars(&options.input_string);
        let essential = primes::essential_chars(&options.input_string);
        let primes = primes::primes(&essential)?;
        let primes_product = primes::primes_product(&primes)?;
        Ok(Session{lang: lang.clone(), dict_file_path, max_phrase_words,
                   iso_8859_1: *iso_8859_1,
                   skip_upcase: *skip_upcase, skip_short: *skip_short,
                   verbose: *verbose, input_string: input_string.clone(),
                   input_phrase, pattern, essential, primes, primes_product})
    }
}

fn resolve_single(session: &Session) -> Result<(), ErrorKind> {
    if session.verbose {
        println!("filter based upon rules for lang={:?}", session.lang);
        println!("input phrase: {}", &session.input_string);
        println!("pattern: {}", &session.pattern);
        println!("essential-chars: {}", &session.essential);
        println!("primes: {:?}", &session.primes);
        println!("primes-product: {} ({} bits)",
                 &session.primes_product, session.primes_product.bits());
    }
    let mut map: Map = BTreeMap::new();
    let mut wordlist: Vec<String> = vec![];
    for file_path in &session.dict_file_path {
        load_wordlist(&mut wordlist, &mut map, &file_path, &session)?;
    }
    if !wordlist.is_empty() {
        if session.verbose {
            println!("\nCandidate single words:\n");
        }
        println!("{:?}", wordlist);
    }
    if session.max_phrase_words == 0 || session.max_phrase_words > 1 {
        if session.verbose {
            println!("\nCandidate phrases:\n");
        }
        let results = search::brute_force(&session.primes_product, &map,
                                          session.max_phrase_words);
        let results = results.0; // unpack tuple struct, Candidate
        if session.verbose {
            println!("Results={}", results.len());
        }
        for terms in &results {
            if terms.len() == 2 {
                println!("{:?}", terms);
            }
        }
        println!("");
        for terms in &results {
            if terms.len() == 3 {
                println!("{:?}", terms);
            }
        }
        println!("");
        for terms in &results {
            if terms.len() > 3 {
                println!("{:?}", terms);
            }
        }
    }
    Ok(())
}

/// Load dictionary of natural language words (e.g., English) for
/// possible inclusion when searching combinations of words for
/// constructing candidate anagrams that match the input phrase.
/// Reject dictionary words based upon various criteria: 1) too long
/// to possibly match; 2) containing characters other than those from
/// the input pattern; 3) words where their product is greater than
/// that of the input phrase.
/// Params `wordlist` is set of single word matches, and `map` is a
/// tree containing dictionary words selected after initial filtering.
/// Other parameters are same as their namesakes from
/// `primes::filter_word()`.
/// SIDE-EFFECTS: `wordlist` and `map` will likely be updated.
fn load_wordlist(wordlist: &mut Vec<String>, map: &mut Map, filepath: &str,
                 session: &Session) -> Result<(), ErrorKind> {
    let input_length = session.essential.len();
    let empty: Vec<&str> = vec![];
    let short_words = SHORT.get(&session.lang).unwrap_or(&empty);
    let upcase_words = UPCASE.get(&session.lang).unwrap_or(&empty);
    let f = File::open(filepath)
        .map_err(|e| {println!("Unable to open: {}", filepath); e})?;
    let mut f = BufReader::new(f);
    let mut bytes: Vec<u8> = vec![];
    let mut word = String::new();
    let mut previous = String::new();
    let mut i = 0;
    loop {
        i += 1;
        bytes.clear();
        word.clear();
        match f.read_until(0x0A, &mut bytes) {
            Ok(0) => break,     // End of file (EOF)
            Ok(_n) => {
                if session.iso_8859_1 {
                    word = bytes.iter().map(|&x| char::from(x)).collect();
                } else {
                    word = String::from_utf8_lossy(&bytes).to_string();
                }
                word = word.trim().to_string();
                if word.is_empty() {
                    continue
                }
                if word == previous { // some dictionaries contain duplicates
                    continue
                }
                if per_lang_filter(&word, &session, &short_words, &upcase_words) {
                    continue
                }
                if session.input_phrase.iter().any(|&x| x == word) {
                    continue    // filter words from input phrase
                }
                previous = word.clone();
                if let Ok(product) = primes::filter_word(&word, &session.pattern,
                                                         input_length,
                                                         &session.primes_product) {
                    if product == session.primes_product { // single word match
                        wordlist.push(word.to_string());
                    } else { // Store remaining words in look-up table:
                        map.entry(product)
                            .or_insert(Vec::with_capacity(1))
                            .push(word.to_string());
                    }
                }
            }
            Err(e) => {
                println!("file error: {:?}", e);
                break
            }
        }
    }
    if session.verbose {
        println!("Dictionary word list: lines={}, filtered-entries={}",
                 i, map.len());
    }
    Ok(())
}

/// Language-specific filtering for words from dictionary.
/// See `languages::*` for supplying `short_words` and `upcase_words`.
/// Return value indicates whether to reject dictionary `word` or not.
#[inline]
fn per_lang_filter(word: &str, session: &Session,
                   short_words: &[&str], upcase_words: &[&str]) -> bool {
    if session.lang == Available::Any {
        if session.skip_short && word.len() < 2 {
            true
        }
        else if session.skip_upcase && word.find(char::is_uppercase).is_some() {
            true
        } else {
            false
        }
    } else {
        if word.len() == 1 {
            if short_words.iter().any(|&w| w == word) {
                false
            } else {
                true
            }
        } else if word.find(char::is_uppercase).is_some() {
            if upcase_words.iter().any(|&w| w == word) {
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}
