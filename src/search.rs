use num_bigint::BigUint;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::config::Config;
use crate::error::Result;
use crate::primes;
use crate::words::Cache;

/// Values computed from each query.
/// See also: [Session] and bin/anagrams.rs CLI Options.
///
/// NOTE: Multiple languages within same run-time are accommodated as
/// an implementation detail of each app; e.g., CLI app filters while
/// loading a transient dictionary, but HTTP service caches each dict
/// and then filters per query.
#[derive(Debug)]
pub struct Search<'a, 'b> {
    /// Query terms (words) after parsing for whitespace
    pub input_phrase: &'a [String],
    /// Results must include this set of words.
    pub must_include: &'a [String],

    /// Set of unique characters extracted from query
    pub pattern: String,
    /// Alphabetic characters of query including duplicates
    pub essential: String,
    /// Set of prime numbers corresponding to `essential`
    pub primes: Vec<u16>,
    /// Grand total computed from all values in `primes`
    pub primes_product: BigUint,

    /// Configuration with any per-query override values
    pub config: &'b Config,
}

impl<'a, 'b> Search<'a, 'b> {
    /// Fallible constructor where search query is supplied.
    /// Computes metadata.
    /// Next, call fn [factors] via chaining.
    pub fn query(
        input_phrase: &'a [String], must_include: &'a [String], config: &'b Config,
    ) -> Result<Search<'a, 'b>> {
        let input_string = input_phrase.join("");
        let pattern = primes::extract_unique_chars(&input_string);
        let essential = primes::essential_chars(&input_string);
        let primes = primes::primes(&essential)?;
        let mut primes_product = primes::primes_product(&primes)?;

        if !must_include.is_empty() {
            let s = must_include.join("");
            let e = primes::essential_chars(&s);
            let p = primes::primes(&e)?;
            let denominator = primes::primes_product(&p)?;
            primes_product /= denominator;
        }

        Ok(Search {
            input_phrase,
            must_include,
            pattern,
            essential,
            primes,
            primes_product,
            config,
        })
    }

    /// Next, call fn [factors] via chaining.
    pub fn add_cache(&'a self, cache: &'b Cache) -> SearchBuilder {
        SearchBuilder::new(self, cache)
    }
}

/// Internal state that may be mutated while performing a search.
///
/// Associate a product of primes with its word list from the
/// dictionary file loaded at program start.
///
/// The `accumulator` is similar to values within primes::Map type but
/// as references to those values.  Similarly for `results` being a
/// vector as set of previous successful `accumulator` instances.
///
/// The `accumulator` gets populated to completion of a single
/// candidate phrase that is an anagram for the input phrase;
/// otherwise, contents get discarded.
pub struct SearchBuilder<'a, 'b> {
    query: &'b Search<'a, 'b>,

    /// Cache of word list and its metadata
    dict: &'b Cache<'b>,

    /// Candidate words in incomplete phrase
    /// where inner array is list of words from dictionary with same product.
    accumulator: Vec<&'b [String]>,
    /// BTree to ensure unique phrases when complete
    dedup: BTreeMap<String, bool>,
    /// Final set of unique phrases from which post-processing may be done
    results: Candidate<'b>,
}

/// Query results (but NOT named "result" so as to avoid confusion
/// with [std::error::Result]).
///
/// Candidate phrases that are anagrams of the input phrase.
/// These are candidates requiring further evaluation such as by a
/// human to select or be verified by NLP Parts-of-Speech tagging, etc.
#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Candidate<'a>(pub Vec<Vec<&'a [String]>>);

impl<'a, 'b> SearchBuilder<'a, 'b>
where
    'a: 'b,
{
    /// Initiate recursion (albeit, limited iterations) with proper
    /// starting values.
    /// Usage:
    /// ```ignore
    /// let search = Search::query(...)?;
    /// let (dict, _) = words::load_and_select(...)?;
    /// let results = search.add_cache(&dict).brute_force();
    /// ```
    pub fn brute_force(&mut self) -> Candidate<'b> {
        let initial_value = &self.query.primes_product;
        let max_recursion = self.query.config.max_phrase_words;
        self.factors(initial_value, 0, max_recursion);
        self.results.to_owned()
    }

    /// Internal constructor.
    /// See impl [Search] or fn [brute_force] instead.
    fn new(query: &'b Search<'a, 'b>, cache: &'b Cache) -> SearchBuilder<'a, 'b> {
        let accumulator =
            if query.must_include.is_empty() { vec![] } else { vec![query.must_include] };
        SearchBuilder {
            query,
            dict: cache,
            accumulator,
            dedup: BTreeMap::new(),
            results: Candidate(vec![]),
        }
    }

    /// Find words in dictionary based upon prime number factorization.
    /// This is recursive with few iterations, albeit long iterations.
    ///
    /// Params: `product` is the remaining primes product of input
    /// phrase, and `recursion_depth` is maximum number of words to be
    /// allowed in candidate result phrase.

    // Rules of thumb for recursion-- think in terms of a state-machine
    // that handles:
    //  1. null case; i.e., terminate recursion
    //  2. special cases; e.g., identity, n*1, etc.
    //  3. the general case

    // Implementation notes: Loading of dictionary word list tests for
    // exact match of products in CLI version, and similar lookup
    // against cache in HTTP version, so logic here builds upon the
    // ASSUMPTION that single word results have already been resolved.

    // There's no Tail Call Optimization as of Rust v1.35 [or 1.80) and
    // unlikely any time soon, so this violates conventional practice
    // by having other logic after a recursive call-- for readability.
    fn factors(&mut self, product: &BigUint, start: usize, recursion_depth: usize) {
        let limit = self.dict.descending_keys.len();
        if start >= limit {
            return;
        }
        let mut i = start;
        while i != limit {
            let test_product = self.dict.descending_keys[i];
            let words = self.dict.lexicon.get(test_product).unwrap();
            i += 1;
            if product == test_product {
                // Exact match -- Execution only reaches here via recursion
                self.accumulator.push(words);
                // Success: only one key in `dictionary` could match `product`
                self.push_if_unique();
                return;
            } else if product > test_product && product % test_product == BigUint::ZERO {
                // Found a factor that fits chain within accumulator.
                // Optimization to possibly avoid recursion + loop:
                let remainder = product / test_product;
                if let Some(more_words) = self.dict.lexicon.get(&remainder) {
                    self.accumulator.push(words);
                    self.accumulator.push(more_words);
                    self.push_if_unique();
                    if start > 0 {
                        // Execution reached here via recursion
                        return;
                    }
                } else if recursion_depth > 1 {
                    // already checked 1 word remainder
                    self.accumulator.push(words);
                    // Avoid processing same entries; `i` already incremented
                    self.factors(&remainder, i, recursion_depth - 1);
                    if start > 0 {
                        // Execution reached here via recursion
                        if self.query.must_include.is_empty() {
                            self.accumulator.clear();
                        } else {
                            self.accumulator = vec![self.query.must_include];
                        }
                        return;
                    }
                }
            }
        }
        if self.query.must_include.is_empty() {
            self.accumulator.clear();
        } else {
            self.accumulator = vec![self.query.must_include];
        }
    }

    /// De-duplicate candidate anagram results by sorting words within
    /// a phrase and storing the sorted phrase within an instance of
    /// this tree.
    /// Let there be one instance of a phrase within results,
    /// regardless of word order.
    /// SIDE-EFFECTS: consumes `self.accumulator` completely, and
    /// possibly updates `self.results`.
    #[allow(clippy::map_entry)]
    fn push_if_unique(&mut self) {
        // Arrange (first) words within phrase in alphabetical order:
        self.accumulator.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let string: String = self
            .accumulator
            .iter()
            .map(|&x| x[0].as_str())
            .collect::<Vec<&str>>()
            .join("");
        // Avoid the Entry API because this clears instead of updates.
        if self.dedup.contains_key(&string) {
            if self.query.must_include.is_empty() {
                self.accumulator.clear();
            } else {
                self.accumulator = vec![self.query.must_include];
            }
        } else {
            self.dedup.insert(string, true);
            self.results.0.push(self.accumulator.drain(..).collect());
            if !self.query.must_include.is_empty() {
                self.accumulator = vec![self.query.must_include];
            }
        }
    }
}
