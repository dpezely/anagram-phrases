Change Log
==========

## Overview

Being pre-1.0, backwards compatibility with older versions may break, as is
the case with v0.5 compared to earlier versions.

As of v0.5.0, performance remains constrained due to *lack* of concurrency,
and number of anagrams found significantly increased since v0.4.  Minimal
concurrency is introduced with v0.6 for streaming results to the CLI but
designed specifically for future WebSocket behavior.  (Concurrent workers
are yet to come.)

Those factors-- compounded by processing long words and/or long phrases--
can lead to long running time.

Compared to versions prior to v0.5.0, an increase of over `80x` has been
found for four word phrases comprised of twenty letters.  The duration for
that same query increased from a few seconds to nearly thirteen minutes on
the same machine.  However, as of v0.6, meaningful results are streamed as
each anagram is found, and the vast majority of the time is exhaustively
confirming that no further phrases can be found.

There probably are algorithmic optimizations that can be applied which would
greatly reduce duration of that final stage.

## v0.6.0 - Streaming Results & Writing JSON

This release introduces concurrency but only for producing and consuming
streaming results as each new anagram (or transposition) is found.  These
are baby steps towards functionality accommodating HTTP service as a client
of this library; e.g., for its future release to utilize WebSockets in a
meaningful way.  (More meaningful concurrency across all processors is
planned.)

New features:

- API of `SearchBuilder` accommodates an MPSC channel for streaming results
  + Each phrase gets sent as `Option::Some` via channel as it is found
  + Receiving `Option::None` indicates the listener may exit
  + Channel messages may get wrapped within an different Enum before v1.0
    to accommodate services such as WebSocket
- CLI adds `--json` (`-j`) option for writing sorted results in JSON format
  + Isolates single word results ("transpositions" actually) from phrases
    (proper "anagrams")
  + Each is sorted alphabetically
  + Anagrams are further sorted by number of words in phrase
- CLI adds `--quiet` (`-q`) flag to omit streaming results as each is found
  + Verbose and quiet modes compete, and the last flag specified wins

Fixes:

- Honor *CLI default* for advertised number of words in results
  + v0.5 resolved a programming error only within the search algorithm
  + This error was within the CLI porcelain
- More meaningful `--help` messages
  + Message banner comes from doc-comment, which has been revised
  + Behavior changes and CLI parameters changed in v0.5 didn't correspond
    correctly with help text that was being displayed

### Blocker For 1.0 Release

Known defect with remedy applied; i.e., a bug and a hack:

As noted in a `TODO` within [search.rs](src/search.rs), some runs take way
too long since v0.5.0.  Unless specifying `-D` for max duration, it defaults
to a reasonably short value accommodating older machines, but it's a hack.

Full results arrive within a few seconds. (Finds 100% within those few
seconds in casual testing on AMD Ryzen 5 7535U, which is a 2022 laptop-grade
CPU.)  However, some runs take 10, 20+ minutes to complete.

Limiting elapsed time is pragmatic but a hack nonetheless.  After fixing
this defect, the feature will be kept for forthcoming HTTP service workers.

Deeper investigation will happen when schedule permits.

## v0.5.0 - Correctness of Algorithm

This release breaks backwards compatibility, overhauls the library and
promotes the CLI from hackathon/demonstration app to offer proper utility.

Original implementation of the primes factorization algorithm was done at
a Vancouver Rust Meetup based upon the topic of "anagrams" suggested by a
member during that same meeting, and practically no tuning occurred beyond
de-duplication (061ebc5 as fifth commit) until this release (31st commit).

Like database schema and the *fallacy* of "schemaless," states *always*
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
  + Edit: there were 2 defects, and this addressed only one; see v0.6.0
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
