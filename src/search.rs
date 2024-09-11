use num_bigint::BigUint;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::primes::Map;

/// Associate a product of primes with its word list from the
/// dictionary file loaded at program start.
///
/// The `accumulator` is similar to values within primes::Map type but
/// as references to those values.  Similarly for `results` being a
/// vector as set of previous successful `accumulator` instances.
///
/// Intentions for `accumulator` are that-- per instance-- it gets
/// populated to completion of a single candidate phrase that is an
/// anagram for the input phrase; otherwise, contents get discarded.
#[rustfmt::skip]
struct Search<'a> {
    dictionary: &'a Map,               // keys (sorted) go in descending_keys
    limit: usize,                      // keys.len()
    descending_keys: Vec<&'a BigUint>, // high-to-low enforced by Constructor
    accumulator: Vec<&'a Vec<String>>, // candidate words in incomplete phrase
    dedup: BTreeMap<String, bool>,     // ensure unique phrase when complete
    results: Candidate<'a>,            // final set of unique phrases
}

/// Candidate phrases that are anagrams for the input phrase.
/// These are candidates requiring further evaluation such as by a
/// human to select or perhaps be verified by an MD5 checksum, etc.
#[derive(Serialize, Debug)]
#[serde(transparent)]
pub struct Candidate<'a>(pub Vec<Vec<&'a Vec<String>>>);

/// Iterate through list of remaining words within `map`, given that
/// dictionary words within it have been selected as viable partial
/// matches based upon prime number factorization.  After factoring
/// each word's product from the input phrase, 1) check for exact
/// match of remaining factor within `map` for a two word result; 2)
/// test each word's product to see if it's a factor of the remaining
/// factor within `map` for possible n-word result.
pub fn brute_force<'a>(
    primes_product: &BigUint, map: &'a Map, max_phrase_words: usize,
) -> Candidate<'a> {
    let mut search = Search::new(map);
    search.factors(primes_product, 0, max_phrase_words);
    search.results
}

impl<'a, 'b> Search<'a> {
    /// Constructor
    fn new(map: &'a Map) -> Self {
        let mut keys: Vec<&BigUint> = map.keys().collect();
        keys.sort_by(|a, b| b.cmp(a));
        assert!(keys[0] > keys[keys.len() - 1]);
        Search {
            dictionary: map,
            limit: keys.len(),
            descending_keys: keys,
            accumulator: vec![],
            dedup: BTreeMap::new(),
            results: Candidate(vec![]),
        }
    }

    /// Find words in dictionary based upon prime number factorization.
    /// This is recursive with few iterations, albeit long iterations.
    /// Params: `product` is primes product of input phrase, `map` is
    /// filtered dictionary word list, `keys` are from map while
    /// pre-sorted large to small, and `recursion_depth` is maximum number
    /// of words to be allowed in candidate result phrase.

    // Rules of thumb for recursion-- think in terms of a state-machine
    // that handles:
    //  1. null case; i.e., terminate recursion
    //  2. special cases; e.g., identity, n*1, etc.
    //  3. the general case

    // Implementation notes: Loading of dictionary word list tests for
    // exact match of products, so logic here builds upon that assumption.

    // There's no Tail Call Optimizations as of Rust v1.35 [or 1.80) and
    // unlikely any time soon, so this violates conventional practice
    // by having other logic after a recursive call-- for readability.
    fn factors(&mut self, product: &'b BigUint, start: usize, recursion_depth: usize) {
        if start >= self.limit {
            return;
        }
        let mut i = start;
        while i != self.limit {
            let test_product = self.descending_keys[i];
            let words = self.dictionary.get(test_product).unwrap();
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
                if let Some(more_words) = self.dictionary.get(&remainder) {
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
                        self.accumulator.clear();
                        return;
                    }
                }
            }
        }
        self.accumulator.clear();
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
            self.accumulator.clear();
        } else {
            self.dedup.insert(string, true);
            self.results.0.push(self.accumulator.drain(..).collect());
        }
    }
}
