//! Command-line interface for solving anagrams and allows multiple
//! words for input and results.
//!
//! This is merely porcelain, and as much functionality/plumbing as
//! can be shared with HTTP service lives in the library.

#[cfg(feature = "external-hasher")]
extern crate char_seq;

extern crate anagram_phrases;

use clap::Parser;
use std::convert::From;

use anagram_phrases::config::Config;
use anagram_phrases::error::Result;
use anagram_phrases::search::Search;
use anagram_phrases::words;

/// Query and runtime options.
///
/// See also: [crate::search::Search].
#[derive(Debug, Parser)]
#[clap(max_term_width = 80)]
struct Session {
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
    let session = Session::parse();
    if session.verbose {
        // TODO set env log level
        println!("filter based upon rules for lang={:?}", session.config.lang);
        println!("input phrase: {}", &session.input_phrase.join(" "));
    }
    let max_phrase_words = match session.config.max_phrase_words {
        0 => std::cmp::min(session.input_phrase.len() + 1, 3),
        n if n < 3 => 3,
        n => n,
    };
    let session =
        Session { config: Config { max_phrase_words, ..session.config }, ..session };

    let search = Search::query(&session.input_phrase, &session.config)?;
    let (dict, singles) = words::load_and_select(
        &session.config,
        &search.pattern,
        &search.essential,
        &search.primes_product,
    )?;
    if session.verbose {
        println!("pattern: {}", &search.pattern);
        println!("essential-chars: {}", &search.essential);
        println!("primes: {:?}", &search.primes);
        println!(
            "primes-product: {} ({} bits)",
            &search.primes_product,
            &search.primes_product.bits()
        );
        println!("maximum number of words in result phrase: {max_phrase_words}");
    }
    if !singles.is_empty() {
        if session.verbose {
            println!("\nCandidate single words:\n");
        }
        println!("{:?}", singles);
    }
    // When max words is exactly one (a transposition, not anagram/phrase),
    // it would have been found above while loading dictionary.
    if session.config.max_phrase_words != 1 {
        let cache = words::Cache::init(&dict);
        let results = search.add_cache(&cache).brute_force();
        let results = results.0; // unpack tuple struct, Candidate
        if session.verbose {
            println!("\nCandidate phrases:\n");
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Session::command().debug_assert()
    }
}
