Change Log
==========

## v0.5.0 - Correctness

This release breaks backwards compatibility, overhauls the library and
promotes the CLI from hackathon/demonstration app to offer proper utility.

Original implementation of the primes factorization algorithm was done at
a Vancouver Rust Meetup based upon the topic of "anagrams" suggested by a
member during that same meeting, and practically no tuning occurred beyond
de-duplication (061ebc5 as fifth commit) until this release (31st commit).

Like database schema and the *fallacy* of "schema-less," states *always*
exist; therefore, keep Finite State Machines (FSM) explicit for anything
that matters.  Then when in doubt, revert to a well-known stable state;
e.g., Erlang's "Let it crash" motto.

Following my own advice elevates this from toy/throwaway project 5yrs later.

New features:

- Library contains nearly all functionality/plumbing
- CLI versus HTTP service, etc. each intended merely as a facade/porcelain
- CLI options: specify words that *must exist* and/or be *omitted* within
  each phrase of results

Behavior changes / breaking changes:

- RESULTS MAY DIFFER because of corrections to v0.1.0 - 0.4.0 core algorithm
    + As a phrase accumulates each word, branching now occurs
    + Each branch can find different chains of words comprising an anagram
- Public and private APIs:
    + `Session` obsolete: its structs and impls migrated to `search::Search`
      because some fields were redundant with `Options` in `bin/anagrams.rs`,
      and computed values became internal state of `search::SearchBuilder`
    + Explicit internal (private) `State` tracks progress of accumulated
      phrases until `State:;Complete` representing an anagram
- CLI arguments `--short` (-s) and `--upcase` (-u) specify *inclusion*
    + Previously, args with same name indicated the opposite
- Obsoletes `GitLAB.com/dpezely/native-android-kotlin-rust`
    + by same author as proof of concept; maybe it helped someone out there
    + 'twas early days of one Rust library across CLI, httpd, mobile apps
- Upgrades `clap` to 4.5, thus removes legacy `structopt`, leverages `flatten`
- Requires Rust 1.80 or newer because `LazyLock` supersedes `lazy-static`

Fixes:

- Honor specified maximum number of words in results
- Successful anagrams can contain repeated words
- Many more anagrams can be found compared to early versions

## v0.4.0 - Edition 2021 maintenance release

- Minor updates to wording within README
- Resolve findings from `cargo +nightly clippy` as of rustc 1.67.1
- Update to newer revisions of libraries
- Replace obsolete/unmaintained dependencies suggested by `cargo audit`
- Upgrade to Edition 2021: `cargo fix --edition`

## v0.3.0 - Library

- Accommodates being used as library/API such as for web service,
  [anagram-phrases-httpd](https://gitlab.com/dpezely/anagram-phrases-httpd),
  by same author.
- Adds some function tests but could benefit by more bracketing, more tests.
- Removes and ignores Cargo.lock from Git repo, because this is a library now.
- Creating command-line interface executable now requires an explicit flag:  
  `cargo build --release --bin anagram-phrases`

## v0.2.0 - De-duplication Within Results

- De-duplicates results with slight performance degradation;
  see `search::Search::push_if_unique()` for details.
- Fixes a minor programming error (bug) by deferring inclusion of candidate
  words into phrase word list accumulator until *after* confirming that the
  follow-on step also extends the accumulator, so now results beyond initial
  1-3 word phrases are also valid.

## v0.1.0 - Preliminary Release As Command-Line Interface

- Loads & filters dictionary word list for pruned search space, then
  iterates through that pre-sorted (descending sequence) reduced dataset
  with minimal recursion.
- Separates command-line `Options` from `Session` for processing each
  request.
- The set of result phrases remains unsorted.  This maintains the natural
  order where "interesting" phrases (those with words containing larger
  product of primes) emerge at the head of the list.
  + For changing what "interesting" means, supply an alternate
  `primes::hash()` function.  See #overriding-dependencies within
  https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
- Initial support for LANG specific filters, but only English is reliable
  thus far.
- Contains minimal function tests.
- Contains Dockerfile and Makefile for build-only production releases.
- Create command-line interface executable: `cargo build --release`
