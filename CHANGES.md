Change Log
==========

## v0.2.0 - De-duplication Within Results

- De-duplicates results with slight performance degradation;
  see `search::Dedup` for details.
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
- Contains mimimal function tests.
- Contains Dockerfile and Makefile for build-only production releases.
- Create command-line interface executable: `cargo build --release`
