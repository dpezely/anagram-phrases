//! Command-line interface for solving anagrams and allows multiple
//! words for input and results.

#[cfg(feature="external-hasher")]
extern crate char_seq;

extern crate anagram_phrases;

use std::collections::BTreeMap;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;
use structopt::StructOpt;

use anagram_phrases::error::ErrorKind;
use anagram_phrases::languages::{self, Language, SHORT, UPCASE};
use anagram_phrases::primes::{self, Map};
use anagram_phrases::search;
use anagram_phrases::session::Session;

#[derive(StructOpt, Debug)]
#[structopt(max_term_width=80)]
struct Options {
    /// Specify 2 letter ISO code for natural language such as EN for
    /// English, FR for Français, etc. to enable specific filters.
    #[structopt(short="l", long="lang", required=false, default_value="Any",
                raw(possible_values="&Language::variants()",
                    case_insensitive="true"))]
    lang: Language,

    /// Must be a plain-text file containing one word per line.
    /// Files suitable for `ispell` or GNU `aspell` are compatible.
    #[structopt(short="d", long="dict", multiple=true, number_of_values=1,
                default_value="/usr/share/dict/words")]
    dict_file_paths: Vec<String>,

    /// Defaults to one more than number of words within input phrase
    /// and a minimum of 3 words.
    #[structopt(short="m", long="max", default_value="0")]
    max_phrase_words: usize,

    /// Skip dictionary words containing uppercase, which indicates
    /// being a proper names.  However, use --lang=EN to allow "I" as
    /// an exception for English; etc.
    #[structopt(short="u", long="upcase")]
    skip_upcase: bool,

    /// Skip dictionary words containing single letters, which may
    /// help avoid noisy results.  However, use --lang=en allowing
    /// only `a` for English, `y` for Spanish, etc.
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

    /// Currently, only ASCII and ISO-8859-* are supported.
    /// May be a single word or phrase consisting of multiple words.
    /// For a phrase, be sure to use quotes or escape spaces.
    #[structopt(name="PHRASE")]
    input_string: String,
}

/// Resolve a single anagram phrase or word from command-line parameters.
fn main() -> Result<(), ErrorKind> {
    let opts = Options::from_args();
    let Options{lang, dict_file_paths, iso_8859_1, max_phrase_words,
                skip_upcase, skip_short, verbose, input_string} = opts;
    let session =
        Session::start(&lang, dict_file_paths, iso_8859_1, max_phrase_words,
                       skip_upcase, skip_short, verbose, &input_string)?;
    resolve_single(&session)?;
    Ok(())
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
    for file_path in &session.dict_file_paths {
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
fn load_wordlist(wordlist: &mut Vec<String>, map: &mut Map, filepath: &str,
                 session: &Session) -> Result<(), ErrorKind> {
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
                if languages::filter(&word, &short_words, &upcase_words,
                                    session.skip_short, session.skip_upcase) {
                    continue
                }
                if session.input_phrase.iter().any(|&x| x == word) {
                    continue    // filter words from input phrase
                }
                if let Ok(product) = primes::filter_word(&word, &session.pattern,
                                                         input_length,
                                                         &session.primes_product) {
                    if product == session.primes_product { // single word match
                        wordlist.push(word.to_string());
                    } else { // Store remaining words in look-up table:
                        // FIXME: utilize PUSH-NEW semantics
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
