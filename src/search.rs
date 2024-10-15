use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::identities::One;
use serde::Serialize;
use std::collections::{btree_map::Entry, BTreeMap, VecDeque};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::error::Result;
use crate::primes;
use crate::words::Cache;

/// Values computed from each query.
/// See also: bin/anagrams.rs CLI Options.
///
/// NOTE: Multiple languages within same run-time are accommodated as
/// an implementation detail of each app; e.g., CLI app filters while
/// loading a transient dictionary, but HTTP service caches each dict
/// and then filters per query.
#[derive(Clone, Debug)]
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

impl<'a, 'b, 'c> Search<'a, 'b>
where
    'c: 'b,
{
    /// Fallible constructor where search query is supplied.
    /// Computes metadata.
    ///
    /// Usage:
    /// ```ignore
    /// let search = Search::query(...)?;
    /// let (dict, _) = words::load_and_select(...)?;
    /// let cache = words::Cache::init(&dict);
    /// let mut builder = search.enrich(&cache, None);
    /// let mut anagrams = builder.brute_force();
    /// ```
    ///
    /// Enrichment occurs after calling this constructor due to values
    /// computed by it may be necessary when populating [Cache].
    /// i.e., the "builder" pattern; see: [enrich] and [SearchBuilder].
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

    /// Enrich [Search] prior to exercising query.  Add reference to
    /// word list and its metadata. Optionally, add [Sender] side of
    /// MPSC channel.
    ///
    /// The `cache` parameter is the value returned by fn [Cache::init].
    pub fn enrich(
        &'c self, cache: &'b Cache, tx: Option<Sender<UniqueAnagram>>,
        max_duration: Option<Duration>,
    ) -> SearchBuilder<'a, 'b> {
        SearchBuilder { query: self, dict: cache, tx, max_duration }
    }

    /// Add reference to word list and its metadata.
    ///
    /// The `cache`  parameter is the value returned by fn [Cache::init].
    ///
    /// See also fn [enrich].
    pub fn add_cache(&'c self, cache: &'b Cache) -> SearchBuilder<'a, 'b> {
        SearchBuilder { query: self, dict: cache, tx: None, max_duration: None }
    }
}

/// Envelope for sending each new unique anagram as it is found.
/// Follows similar semantics of how [Iterator] uses [Option] but adds
/// an indication of progress.
///
/// Closing the [Sender] should be sufficient signal for listener to
/// end, but `for msg in rx {}` proved otherwise.
pub type UniqueAnagram = Option<Vec<Vec<String>>>;

/// Augment an instance of [Search] with [Cache] and channel [Sender].
#[derive(Clone)]
pub struct SearchBuilder<'a, 'b> {
    /// Includes `input_phrase` and parameters
    query: &'b Search<'a, 'b>,

    /// Cache of word list and its metadata
    dict: &'b Cache<'b>,

    /// Transmits stream of unique anagram phrases as each is found
    tx: Option<Sender<UniqueAnagram>>,

    /// Expire after time elapses (time-to-live, TTL)
    max_duration: Option<Duration>,
}

impl<'a, 'b, 'c> SearchBuilder<'a, 'b>
where
    'c: 'b,
{
    /// Constructor intended to be used by [Search].
    ///
    /// See impl [Search] or fn [brute_force] instead.
    // The real reason why this constructor exists is keeping lifetime
    // *names* consistent across the various structs and impl blocks;
    // i.e., 'c. Otherwise, it's simple enough instantiating struct inline.
    pub fn new(
        query: &'b Search<'a, 'b>, cache: &'b Cache, tx: Option<Sender<UniqueAnagram>>,
        max_duration: Option<Duration>,
    ) -> SearchBuilder<'a, 'b> {
        SearchBuilder { query, dict: cache, tx, max_duration }
    }

    /// Exercise combinations and permutations of dictionary words to
    /// fit within a single phrase such that the product of its set of
    /// prime numbers match that of the query.
    ///
    /// The search space can be pruned in advance when word list gets
    /// loaded on-demand per query ensuring fewer iterations here.
    // TODO divvy into non-overlapping chunks and dispach concurrent workers.
    // TODO full results arrive within a few seconds (100% in casual testing),
    // but some runs take 10, 20, 40+ minutes to complete.
    // Limiting elapsed time is pragmatic but a hack nonetheless.
    // After fixing that defect, keep the feature for HTTP service workers.
    pub fn brute_force(&'c self) -> Vec<Vec<Vec<String>>> {
        let task = Task::new(self);
        let mut results = Candidate::new();
        let time = Instant::now();
        let limit = self.dict.descending_keys.len();
        let mut deque = VecDeque::<Task<'a, 'b>>::with_capacity(limit * 2);
        // TODO allocates new accumulaters, each with one word spanning entire dict
        for i in 0..limit {
            let state = task.clone().factor_i(i);
            match state {
                State::Unchanged(task) => deque.push_back(task),
                State::Reject => {}
                State::Complete((task, mut anagram)) => {
                    deque.push_back(task);
                    if let Some(p) = results.push_if_unique(&mut anagram.phrase) {
                        if let Some(tx) = &self.tx {
                            if tx.send(Some(p)).is_err() {
                                break;
                            }
                        }
                    }
                }
                State::Branch((task, new_task)) => {
                    deque.push_back(task);
                    deque.push_back(new_task);
                }
            }
        }
        // Complete each accumulated phrase from above, or reject it.
        while let Some(task) = deque.pop_front() {
            let state = task.clone().factor_i(task.index);
            match state {
                State::Unchanged(task) => deque.push_front(task),
                State::Reject => {}
                State::Complete((task, mut anagram)) => {
                    deque.push_front(task);
                    if let Some(p) = results.push_if_unique(&mut anagram.phrase) {
                        if let Some(tx) = &self.tx {
                            if tx.send(Some(p)).is_err() {
                                break;
                            }
                        }
                    }
                }
                State::Branch((task, new_task)) => {
                    deque.push_front(task);
                    deque.push_front(new_task);
                }
            }
            if let Some(x) = self.max_duration {
                if time.elapsed() > x {
                    break;
                }
            }
        }
        if let Some(tx) = &self.tx {
            let _ = tx.send(None);
        }
        results.phrases()
    }
}

/// Candidate phrases that are anagrams of the input phrase.
///
/// These are results of [Search::query] (but NOT named "result" so as
/// to avoid confusion with [std::error::Result]).
///
/// Each phrase is constructed as nested vectors of strings.  The
/// innermost [Vec] may contain multiple words, each with an identical
/// primes product.
///
/// These are candidates requiring further evaluation such as by a
/// human to select or be verified by NLP Parts-of-Speech tagging, etc.
/// and not guaranteed to be idiomatic for any natural language.
#[derive(Serialize, Debug)]
#[serde(transparent)]
struct Candidate(BTreeMap<String, Vec<Vec<String>>>);

impl<'a> Candidate {
    /// Constructor
    fn new() -> Self {
        Candidate(BTreeMap::new())
    }

    /// Extract and return resulting anagrams, which consumes `self`
    #[inline]
    fn phrases(&mut self) -> Vec<Vec<Vec<String>>> {
        std::mem::take(&mut self.0).into_values().collect()
    }

    /// De-duplicate anagram `phrase` (accumulator) by sorting words
    /// contained within it and pushing that onto a [BTreeMap]
    /// ensuring one instance of a phrase within results, regardless
    /// of word order.
    ///
    /// This is where *references* to dictionary words become
    /// materialized to [String] for reduced memory consumption while
    /// finding phrases, albeit at marginal computation cost here.
    ///
    /// SIDE-EFFECTS: sorts and may consume `phrase`, and may update `self`.
    ///
    /// Return value indicates whether phrase was unique or not, and if so,
    /// supplies [UniqueAnagram] value suitable for channel [Sender].
    #[inline]
    fn push_if_unique(&mut self, phrase: &mut [&'a [String]]) -> UniqueAnagram {
        // Arrange (first) words within phrase in alphabetical order:
        phrase.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let string: String =
            phrase.iter().map(|&x| x[0].to_string()).collect::<Vec<String>>().join(" ");
        let entry = self.0.entry(string);
        match entry {
            Entry::Vacant(e) => {
                let phrase: Vec<Vec<String>> = phrase.iter().map(|&w| w.into()).collect();
                e.insert(phrase.clone());
                Some(phrase)
            }
            Entry::Occupied(_) => None,
        }
    }
}

/// Status of exercising the current phrase
#[derive(Clone)]
enum State<'a, 'b> {
    /// No match found, and no partial match to discard.
    /// Therefore, increment [Task]'s `index` before iterating.
    Unchanged(Task<'a, 'b>),
    /// Discard the incomplete phrase
    Reject,
    /// Word found which matches query, thereby rendering a complete anagram.
    /// When max_words accommodates, however, continue searching
    /// with existing task's base phrase.
    /// Tuple ordering is: existing [Task], completed [Anagram].
    Complete((Task<'a, 'b>, Anagram<'a, 'b>)),
    /// Enqueue a new task, which branches (gets cloned) from current
    /// task, which addresses multiple phrases with an identical word.
    /// Tuple ordering of [Task]s is: existing, new.
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

/// A single unit of work.
///
/// This represents a single iteration of the main loop within the
/// algorithm to find anagrams, but its state and stack may be
/// decoupled such as for concurrency.
#[derive(Clone)]
struct Task<'a, 'b> {
    /// Original query, because multiple queries potentially exist in same queue.
    search: &'b SearchBuilder<'a, 'b>,
    /// Product of primes for query reduced by factoring primes for each word
    /// added to `accumulator`.  Begins equal to [Search::primes_product].
    target: BigUint,
    /// Initial value of `i` from fn [SearchBuilder::brute_force] loop when
    /// this task was scheduled:
    /// For recursion, each new branch begins by repeating `i` for `start`
    /// which addresses cases with repeated words in same phrase.
    index: usize,
    /// Countdown to limit length of phrase within `accumulator`
    max_words: usize,
    /// Candidate words in incomplete phrase where inner [Vec] is set
    /// of words from dictionary with same product.  This gets sorted
    /// and taken to become [Candidate] for final results.
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
    /// and taken to become a [Candidate] for final results.
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
