//! Command-line interface for solving anagrams and allows multiple
//! words for input and results.

#[cfg(feature = "external-hasher")]
extern crate char_seq;

extern crate anagram_phrases;

use clap::Parser;
use std::collections::BTreeMap;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anagram_phrases::config::Config;
use anagram_phrases::error::Result;
use anagram_phrases::languages::{self, Encoding, SHORT, UPCASE};
use anagram_phrases::primes::{self, Map};
use anagram_phrases::search;
use anagram_phrases::session::Session;

/// Query and runtime options.
#[derive(Debug, Parser)]
#[clap(max_term_width = 80)]
struct Options {
    /// One or more words to be resolved as transposition or anagram.
    /// Only ASCII and ISO-8859-* character ranges supported as UTF-8.
    #[clap(name = "WORD", required = true)]
    input_phrase: Vec<String>,

    #[command(flatten)]
    config: Config,

    /// Display additional status information
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

/// Resolve a single anagram phrase or word from command-line parameters.
fn main() -> Result<()> {
    let opts = Options::parse();
    let Options { input_phrase, config, verbose } = opts;
    let Config {
        lang,
        dict_file_paths,
        encoding,
        max_phrase_words,
        include_upcase,
        include_short,
    } = config;
    let skip_upcase = !include_upcase;
    let skip_short = !include_short;
    let session = Session::start(
        &lang,
        &dict_file_paths,
        encoding,
        max_phrase_words,
        skip_upcase,
        skip_short,
        verbose,
        &input_phrase,
    )?;
    resolve_single(&session)?;
    Ok(())
}

fn resolve_single(session: &Session) -> Result<()> {
    if session.verbose {
        println!("filter based upon rules for lang={:?}", session.lang);
        println!("input string: {}", &session.input_string);
        println!("pattern: {}", &session.pattern);
        println!("essential-chars: {}", &session.essential);
        println!("primes: {:?}", &session.primes);
        println!(
            "primes-product: {} ({} bits)",
            &session.primes_product,
            session.primes_product.bits()
        );
    }
    let mut map: Map = BTreeMap::new();
    let mut wordlist: Vec<String> = vec![];
    for file_path in session.dict_file_paths.iter() {
        load_wordlist(&mut wordlist, &mut map, file_path, session)?;
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
        let results =
            search::brute_force(&session.primes_product, &map, session.max_phrase_words);
        let results = results.0; // unpack tuple struct, Candidate
        if session.verbose {
            println!("Results={}", results.len());
        }
        for terms in &results {
            if terms.len() == 2 {
                println!("{:?}", terms);
            }
        }
        println!();
        for terms in &results {
            if terms.len() == 3 {
                println!("{:?}", terms);
            }
        }
        println!();
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
///
/// Reject dictionary words based upon various criteria: 1) too long
/// to possibly match; 2) containing characters other than those from
/// the input pattern; 3) words where their product is greater than
/// that of the input phrase.
///
/// Params `wordlist` is set of single word matches, and `map` is a
/// tree containing dictionary words selected after initial filtering.
/// Other parameters are same as their namesakes from
/// `primes::filter_word()`.
/// SIDE-EFFECTS: `wordlist` and `map` will likely be updated.
#[rustfmt::skip]
fn load_wordlist(wordlist: &mut Vec<String>, map: &mut Map,
                 filepath: &std::path::PathBuf, session: &Session) -> Result<()> {
    let input_length = session.essential.len();
    let empty: Vec<&str> = vec![];
    let short_words = SHORT.get(&session.lang).unwrap_or(&empty);
    let upcase_words = UPCASE.get(&session.lang).unwrap_or(&empty);
    let f = File::open(filepath)?;
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
                if session.encoding == Encoding::Iso_8859_1 {
                    word = bytes.iter().map(|&x| char::from(x)).collect();
                } else {
                    word = String::from_utf8_lossy(&bytes).to_string();
                }
                word = word.trim().to_string();
                if word.is_empty() { continue }
                if word == previous { continue }
                if languages::filter(&word, short_words, upcase_words,
                                    session.skip_short, session.skip_upcase) {
                    continue
                }
                // filter words from input phrase
                if session.input_phrase.iter().any(|x| *x == word) { continue }
                if let Ok(product) = primes::filter_word(&word, &session.pattern,
                                                         input_length,
                                                         &session.primes_product) {
                    if product == session.primes_product {
                        // single word match
                        wordlist.push(word.to_string());
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
                println!("File error: line={} {:?}", i, e);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Options::command().debug_assert()
    }
}
