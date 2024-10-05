use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::identities::One;
use serde::Serialize;
use std::collections::{BTreeMap, VecDeque};

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

    /// Add reference to word list and its metadata.
    ///
    /// The parameter is the value returned by fn [Cache::init].
    ///
    /// Next, call fn [SearchBuilder::factors] via chaining.
    pub fn add_cache(&'a self, cache: &'b Cache) -> SearchBuilder<'a, 'b> {
        SearchBuilder::new(self, cache)
    }
}

/// Augment an instance of [Search] with [words::Cache]
#[derive(Clone)]
pub struct SearchBuilder<'a, 'b> {
    query: &'b Search<'a, 'b>,

    /// Cache of word list and its metadata
    dict: &'b Cache<'b>,
}

impl<'a, 'b, 'c> SearchBuilder<'a, 'b>
where
    'c: 'b,
{
    /// Internal constructor intended to be used by [Search].
    ///
    /// See impl [Search] or fn [brute_force] instead.
    fn new(query: &'b Search<'a, 'b>, cache: &'b Cache) -> SearchBuilder<'a, 'b> {
        SearchBuilder { query, dict: cache }
    }

    /// Exercise *all* *combinations* of words from dictionary to fit
    /// within a single phrase such that the product of its set of
    /// prime numbers match that of the query.
    ///
    /// The search space can be pruned in advance when word list gets
    /// loaded on-demand per query ensuring fewer iterations here.
    ///
    /// Usage:
    /// ```ignore
    /// let search = Search::query(...)?;
    /// let (dict, _) = words::load_and_select(...)?;
    /// let cache = words::Cache::init(&dict);
    /// let mut builder = search.add_cache(&cache);
    /// let mut anagrams = builder.brute_force();
    /// ```
    pub fn brute_force(&'c mut self) -> Vec<Vec<&'b [String]>> {
        let task = Task::new(self);
        let mut results = Candidate::new();
        let limit = self.dict.descending_keys.len();
        if task.index < limit {
            let mut deque = VecDeque::<Task<'a, 'b>>::new();
            for i in task.index..limit {
                let state = task.clone().factor_i(i);
                match state {
                    State::Unchanged(task) => deque.push_back(task),
                    State::Reject => {}
                    State::Complete((task, mut anagram)) => {
                        deque.push_back(task);
                        results.push_if_unique(&mut anagram.phrase);
                    }
                    State::Branch((task, new_task)) => {
                        deque.push_back(task);
                        deque.push_back(new_task);
                    }
                }
            }
            while let Some(task) = deque.pop_front() {
                let state = task.clone().factor_i(task.index);
                match state {
                    State::Unchanged(task) => deque.push_front(task),
                    State::Reject => {}
                    State::Complete((task, mut anagram)) => {
                        deque.push_front(task);
                        results.push_if_unique(&mut anagram.phrase);
                    }
                    State::Branch((task, new_task)) => {
                        deque.push_front(task);
                        deque.push_front(new_task);
                    }
                }
            }
        }
        results.phrases()
    }
}

/// Candidate phrases that are anagrams of the input phrase.
///
/// These are results of [Search::query] (but NOT named "result" so as
/// to avoid confusion with [std::error::Result]).
///
/// Each phrase is constructed as a vector of arrays of strings.  The
/// innermost array may contain multiple words, each with an identical
/// primes product.
///
/// These are candidates requiring further evaluation such as by a
/// human to select or be verified by NLP Parts-of-Speech tagging, etc.
/// and not guaranteed to be idiomatic for any natural language.
#[derive(Serialize, Debug)]
#[serde(transparent)]
struct Candidate<'a>(BTreeMap<String, Vec<&'a [String]>>);

impl<'a> Candidate<'a> {
    /// Constructor
    fn new() -> Self {
        Candidate(BTreeMap::new())
    }

    /// Return the resulting anagrams, which consumes `self`
    fn phrases(&mut self) -> Vec<Vec<&'a [String]>> {
        std::mem::take(&mut self.0).into_values().collect()
    }

    /// De-duplicate anagram `phrase` (accumulator) by sorting words
    /// contained within it and pushing that onto `self.phrases` such
    /// that there will be one instance of a phrase within results,
    /// regardless of word order.
    ///
    /// SIDE-EFFECTS: sorts and may consume `phrase`, and may update `self`.
    fn push_if_unique(&mut self, phrase: &mut Vec<&'a [String]>) {
        // Arrange (first) words within phrase in alphabetical order:
        phrase.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let string: String =
            phrase.iter().map(|&x| x[0].as_str()).collect::<Vec<&str>>().join(" ");
        self.0.entry(string).or_insert(std::mem::take(phrase));
    }
}

/// Status of exercising the current phrase
#[derive(Clone)]
enum State<'a, 'b> {
    /// No match found, and no partial match to discard.
    /// Therefore, increment task's `index` before iterating.
    Unchanged(Task<'a, 'b>),
    /// Discard the incomplete phrase within [Task::accumulator]
    Reject,
    /// Word found which matches query, thereby rendering a complete anagram.
    /// When max_words accommodates, however, continue searching
    /// with existing task's base phrase.
    /// Tuple ordering is: existing task, completed anagram.
    Complete((Task<'a, 'b>, Anagram<'a, 'b>)),
    /// Enqueue a new task, which branches (gets cloned) from current
    /// task, which addresses multiple phrases with identical word.
    /// Tuple ordering of tasks is: existing, new.
    Branch((Task<'a, 'b>, Task<'a, 'b>)),
}

impl<'a, 'b> std::fmt::Debug for State<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unchanged(task) => write!(f, "State::Unchanged:{{{task}}}"),
            State::Complete((task, anagram)) => {
                write!(f, "State::Complete:{{task={task}, anagram={anagram}}}")
            }
            State::Reject => write!(f, "State::Reject"),
            State::Branch((existing, new_task)) => {
                write!(f, "State::Branch:{{existing={existing}, new={new_task}}}")
            }
        }
    }
}

impl<'a, 'b> std::fmt::Display for State<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Unchanged(task) => write!(f, "Unchanged:{task}"),
            State::Complete((task, anagram)) => {
                write!(f, "Complete:task={task}, anagram={anagram}")
            }
            State::Reject => write!(f, "Reject"),
            State::Branch((existing, new_task)) => {
                write!(f, "Branch:existing={existing}, new={new_task}")
            }
        }
    }
}

#[derive(Clone)]
struct Task<'a, 'b> {
    /// Original query, because multiple queries can exist within same queue.
    search: &'b SearchBuilder<'a, 'b>,
    /// Product of primes for query reduced by factoring primes for each word
    /// added to `accumulator`.  Begins equal to [Search::primes_product].
    target: BigUint,
    /// Initial value of `i` from fn [factor] loop when this task was scheduled:
    /// For recursion, each new branch begins by repeating `i` for `start`
    /// which addresses cases with repeated words in same phrase.
    index: usize,
    /// Limit length of phrase within `accumulator`
    max_words: usize,
    /// Candidate words in incomplete phrase where inner array is list
    /// of words from dictionary with same product.  This gets sorted
    /// and then moved to become a [Candidate] for final results.
    accumulator: Vec<&'b [String]>,
    /// Product of all primes within `accumulator` (or default value: 1)
    acc_product: BigUint,
}

impl<'a, 'b> Task<'a, 'b> {
    /// Constructor
    fn new(builder: &'a SearchBuilder<'a, 'b>) -> Self {
        let target = builder.query.primes_product.clone();
        let accumulator = if builder.query.must_include.is_empty() {
            vec![]
        } else {
            vec![builder.query.must_include]
        };
        Task {
            search: builder,
            target,
            index: 0,
            max_words: builder.query.config.max_phrase_words,
            accumulator,
            acc_product: BigUint::one(),
        }
    }

    /// Perform one iteration of factorization.
    ///
    /// Find words in dictionary based upon prime number factorization
    /// where that set of words represents an anagram of the query's
    /// input phrase.
    fn factor_i(self, i: usize) -> State<'a, 'b> {
        if i >= self.search.dict.descending_keys.len() {
            return State::Reject;
        }
        let test_product = self.search.dict.descending_keys[i];
        if test_product > &self.target {
            return State::Unchanged(Task { index: i + 1, ..self });
        }
        // By virtue of `descending_keys` this IF LET will always succeed
        if let Some(words) = self.search.dict.lexicon.get(test_product) {
            let mut accumulator = self.accumulator.clone();
            accumulator.push(words);
            if test_product == &self.target {
                let task = Task { index: i + 1, ..self };
                let anagram = Anagram { search: self.search, phrase: accumulator };
                return State::Complete((task, anagram));
            }
            let acc_product = test_product * &self.acc_product;
            if acc_product == self.search.query.primes_product {
                let task = Task { index: i + 1, ..self };
                let anagram = Anagram { search: self.search, phrase: accumulator };
                return State::Complete((task, anagram));
            }
            // Prepare forking the accumulator which should only be allowed
            // when both the phrase's length and its primes' product allow,
            // but do the computationally cheaper test first (re: BigInt).
            if self.max_words == 1 || acc_product > self.search.query.primes_product {
                return State::Reject;
            }
            // Extend current phrase via branching and without repeating earlier words.
            // (Smaller values of `i` from parent loop were already tried.)
            let (quotient, remainder) = self.target.div_rem(test_product);
            if remainder == BigUint::ZERO {
                let task = Task { index: i + 1, ..self };
                // Continue with same `i` in case of repeated words.
                // Decrement `max_words` due to having pushed `word` above.
                let branch = Task {
                    target: quotient,
                    max_words: self.max_words - 1,
                    accumulator,
                    acc_product,
                    ..self
                };
                return State::Branch((task, branch));
            }
        }
        return State::Unchanged(Task { index: i + 1, ..self });
    }
}

impl<'a, 'b> std::fmt::Debug for Task<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task::{{")?;
        std::fmt::Display::fmt(self, f)?;
        write!(f, "}}")
    }
}

impl<'a, 'b> std::fmt::Display for Task<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "q={} index={} max_words={} acc={:?} p={} vs {}",
            self.search.query.input_phrase.join(" "),
            self.index,
            self.max_words,
            self.accumulator,
            self.acc_product,
            &self.target
        )
    }
}

#[derive(Clone)]
struct Anagram<'a, 'b> {
    /// Original query, because multiple queries can exist within same queue.
    search: &'b SearchBuilder<'a, 'b>,
    /// Completed anagram phrase where inner array is list
    /// of words from dictionary with same product.  This gets sorted
    /// and then moved to become a [Candidate] for final results.
    phrase: Vec<&'b [String]>,
}

impl<'a, 'b> std::fmt::Debug for Anagram<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Anagram::{{")?;
        std::fmt::Display::fmt(self, f)?;
        write!(f, "}}")
    }
}

impl<'a, 'b> std::fmt::Display for Anagram<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "q={} anagram={:?}",
            self.search.query.input_phrase.join(" "),
            self.phrase
        )
    }
}
