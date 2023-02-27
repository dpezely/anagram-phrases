Change Log
==========

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
