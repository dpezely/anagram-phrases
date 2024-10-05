Anagram Phrase Solver Using Primes
==================================

Phrase-based anagram solver using mathematical Prime number factorization:
may be used as a library or command-line utility.

Examples where this package gets used as a library by same author:

- Web service API,
  [anagram-phrases-httpd](https://gitlab.com/dpezely/anagram-phrases-httpd)
- Native Android mobile app using Kotlin, compatible back to Android 4.2
  (API level 17) circa early 2013 devices,
  [native-android-kotlin-rust](https://gitlab.com/dpezely/native-android-kotlin-rust)

This accommodates both single word and multiple word phrases as input, and
both single word *transpositions* and multiple word *anagrams* get generated
as output.

Primes facilitate both pruning the possible search space and using the
product of primes as computed keys for look-up tables.
(See [Background](#background) section for details.)

This tool is ignorant of language semantics and deals only with scripts.

Understand the difference between "language" versus "script" for purposes
here and as defined by the Unicode standard.  Each of English and French is
a natural *language*, yet both use the Latin alphabet.  When applying
accents upon one language's use of otherwise similar characters such as
`'c'` in "Français" or `'e'` in "vérité", then that character set is called
a *script*.  Here, we use "script" and "character set" almost
interchangeably.

## Basic Usage

**This has only be tested on Linux**

This is a command-line utility for handling a single query per run.

Compile using [Rust](http://rust-lang.org/) Edition 2018 or newer, which is
available for BSD Unix, Linux, macOS, Windows and other operating systems:

    cargo build --release

Install its one executable somewhere convenient:

    sudo cp target/release/anagram-phrases /usr/local/bin/

Usage on Debian/Ubuntu and similar flavors of Linux:

    anagram-phrases --help

    anagram-phrases "word or phrase"

Words from your query get excluded from results, but to keep them as
possible words, simply eliminate spaces within the original phrase:

    anagram-phrases wordorphrase

When using a dictionary word list other than `/usr/share/dict/words`:

    anagram-phrases "word or phrase" -d /usr/share/dict/canadian-english-huge

Multiple `-d file-path` options are allowed, and each file will be loaded in
sequence specified.

Input may be a word or phrase with UTF-8 encoding, provided that your shell
accommodates it, such as Bash.

A dictionary word list is **required but not supplied**!

Ones compatible with `ispell` or GNU `aspell` or similar should work without
modification.  On Debian/Ubuntu based Linux systems, look in
`/etc/dictionaries-common/` for where the symbolic link of `words` points,
which is likely `/usr/share/dict/`.  On macOS and FreeBSD, see
`/usr/share/dict/words` and other files within that subdirectory.

Processing of both the input phrase and dictionary word list ignores leading
and trailing white-space as well as non-alphabetic characters.  The
definition of *alphabetic* characters used here comes from the Unicode
implementation within the Rust standard library.

## Building & Running Using Docker

To avoid any contamination of your laptop/workstation's host OS, build and
optionally run using Docker containers.

On Debian/Ubuntu, install using:

    sudo apt-get install docker.io

Or [install](https://download.docker.com/mac/stable/Docker.dmg) for [macOS](https://docs.docker.com/docker-for-mac/docker-toolbox/);
[install](https://docs.docker.com/docker-for-windows/install/) for Windows.

Build: (may require prefixing with `sudo`)

	docker build -f build+run.Dockerfile -t anagrams .

Run:

	docker run -it --rm -v /usr/share/dict:/usr/share/dict anagrams

The `-v` flag above specifies mapping a "volume" for sharing (read-only)
your host's dictionary word list files within the container.  The first
component of that argument corresponds to your host OS's file system and may
be different.

Inside that shell, run:

	anagram-phrases --help

    anagram-phrases --lang=en -d /usr/share/dict/british-english-huge torchwood

Currently, the main caveat is that the `-d` flag should always be specified
to include the dictionary word list, as no default file will be available on
some systems.

See also:
[Dockerfile reference](https://docs.docker.com/engine/reference/builder/).

## Developing

If expanding upon this code, it may be helpful to first apply a
[patch](./debug-search-rs.diff).

This patch adds extremely verbose logging.
(i.e., *who wants to drink from the fire hose?*)

It may be applied from the shell by running:

    patch -p1 < debug-search-rs.diff

The `patch` command is generally available on BSD Unix, Linux, macOS and
Cygwin for Windows.

## Background

The basic principle builds upon prime numbers as exclusive factors of a
value.

That value is the mathematical product of numbers representing alphabetic
characters-- notably, **not** character code-points.

These values map to each letter of the alphabet assigned to a unique prime
number, and all characters of the input phrase are included-- yes,
duplicates too.

By convention, we use primes in sequence: A=2, B=3, C=5, to Z=101 for Latin
scripts such as used by English, but those mappings are arbitrary and
required only to be unique.

For each run, the program creates a subset of the dictionary word list by
filtering for:

1. Any single word longer than the total number of alphabetic characters may
   be rejected.  (This tests number of characters-- not bytes-- because an
   accented letter may resolve to its unaccented equivalent and therefore
   may have different byte counts.)
2. Any word containing a character other than ones within the input phrase
   may be rejected.
3. Any word where its product is greater than the product of the input
   phrase may be rejected.
4. Any single word where its product exactly matches that of the input
   phrase may be collected as a candidate within results and then excluded
   from the search space, having already been consumed.
5. Per-language filters may be optionally applied to eliminate words
   beginning with capital letters or all single letter words, except "I" and
   "a" in English, "y" in Spanish, etc.

With that simple filtering, the search space becomes greatly reduced.


When loading the dictionary list on-demand, memory requirements become
reduced accordingly.  (This describes a single-use command-line utility, so
different criteria and behavior would be used for a persistent web
service.)

For each word remaining within the search space, map each word's list of
characters to primes. (It may contain repeated characters and thus duplicate
primes, which make for a larger product.)

Because this number can potentially exceed `u64` or `u128` type, this
implementation uses [num-bigint](https://crates.io/crates/num-bigint),
written by The Rust Project Developers.

Store the product of all those primes as the key within a hash-table, B-Tree
or similar structure.  This implementation uses
[BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html).


These keys in turn accommodate quickly searching for words to be combined
when constructing candidate anagram phrases equivalent to the input word or
phrase.

Even though the final step is technically a brute-force approach, the search
space will have been aggressively pruned from typically 100 thousand through 300
thousand words and reduced to one tenth of that original number while loading the
dictionary.


> Therefore, time complexity of `O(N*M*logN)` becomes reasonable due to the
> pruned search space (`N`) and phrase length (`M`).  This has been the case
> when N begins above 300k but reduced within N=30 to N=4000 for M=2 and M=4
> word English phrases.
>
> Running time for those have been well *under one second* on a single core
> of i7-8550U CPU with laptop-grade SSD storage for a 25 character, 4 word
> input phrase.
>
> Be sure to **build with `--release` flag**, first.


In addition, this implementation leverages many opportunities for local
optimizations when searching.  One optimization spares an occasional loop
iteration and short-circuits based upon commutative properties of
multiplication.

Duplicates get removed from preliminary result with small performance
penalty of another ephemeral BTreeMap, where keys are words of a
matching phrase sorted and concatenated.

Also note that each word's product gets computed from the *list* of its
primes rather than a *set* because duplicate characters must be tracked.


Selection of the final phrase may be performed by visual inspection for one
that makes sense to the human or being idiomatic within a given natural
language.  (Any additional filtering of results, for example, based upon NLP
Parts-of-Speech tagging is beyond scope of this software.)

## Possible Future Enhancements

To Do:

- [ ] **1.** Accommodate languages beyond those with scripts represented by
  ISO-8859-1 character sets.

  The initial implementation uses `ispell` and GNU `aspell` style dictionary
  files, so in theory any natural language for which comparable word lists
  exist may be supported.  Initially, it's limited to alphabetic range
  within ISO-8859-1, even when loading a dictionary file encoded as UTF-8.

  We map each language's actively used alphabet to a contiguous sequence of
  prime numbers.  Therefore, the full UTF-8 character set is extremely
  problematic for use as-is.

  Local knowledge of each language could provide this mapping of each
  language's script to a compact set of primes.

  A stand-alone [library](https://github.com/dpezely/char-seq/) is already
  in progress and accommodated here as a compiler option.

- [ ] **2.** More interesting presentation of preliminary results may occur,
  by clustering vowels or other high-frequency characters with *larger*
  primes.

  The search loop pre-sorts dictionary words in descending order by product.
  This generally yields chains of longer words first.

  Therefore, more intelligent clustering may reveal more interesting
  results first, for some definition of "interesting".

- [ ] **3.** Integrate with a Natural Language Processing system.

  Select one that reduces words to their lemma form (not word stem) and
  performs parts-of-speech (PoS) tagging while having very efficient
  run-time, such as [spaCy.io](https://spacy.io/).

  Then, obviously bogus candidates might be eliminated while preserving
  novel, silly and valid results.

  (See [Intro to NLP](https://play.org/articles/introduction-to-natural-language-processing)
  article for the least you need to know about the topic and deploying.)

## Understanding Dictionary Word List Dependencies

### Hashing Input & Dictionary Words

All words from input phrase and dictionary word lists get reduced to
lowercase.

Mapping of lowercase characters used in English to prime numbers:

    a,b,c,d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z
    2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101

The built-in hasher also accommodates characters of ISO-8859-1 scripts
generically, which adds code points from U+00A1 through U+00FF, omitting the
non-breaking space character, U+00A0.

This means that there will be unused spans of prime numbers for some
languages.

A separate library facilitates use of UTF-8 for better coverage at some
performance degradation due to additional if/else-if conditionals.  That
feature may be enabled with an optional compiler switch.
See [Cargo.toml](./Cargo.toml) file.

### Using Dictionary Datasets

It's important to understand the available data, so explore the dictionary
word list you wish to use.

Perform similar analysis using the dictionary word list of your choice.
Examples that might be already installed on Linux include:

- /usr/share/dict/words 100k entries with en-US locale defaults

Optional dictionaries offer triple the number of words:

- /usr/share/dict/canadian-english-huge 345k entries

A comprehensive set of word lists for European languages are available as [ispell-dictionaries](https://www.cs.hmc.edu/~geoff/ispell-dictionaries.html).
Files may need to be decompressed first.

Additional dictionaries may be loaded on Debian or Ubuntu flavors of Linux
by running:

	sudo apt-get install \
	  wcanadian-huge \
	  wbritish-huge \
	  wamerican-huge

For manual download of these files, see
[SCOWL (Spell Checker Oriented Word Lists) and
Friends](http://wordlist.sourceforge.net/).

Please see their website for additional versions such as Australian
dictionaries and word frequency reports.

Confirm the character ranges within the dictionary word list:

    grep -c "[^-' a-zA-Z]" wordlist

> 0

Count words with 18+ characters, as this implies large products:

    grep -c '...................' wordlist

> 2085

For dictionary word lists using character sets other than ISO-8859-1, you
can convert them using the separate `iconv` utility.

### Products of Prime Numbers

Knowing the number of "long" words and their proportion to the overall
dictionary word list from the above step indicates how large of integers
might be generated while processing.

The word "superconductivity" may be represented by the following sequence of
primes.  Using Lisp multiplication notation for brevity:

	(* 67 73 53 11 61 5 47 43 7 73 5 71 23 79 23 71 97)

Its product requires 87 bits:

- 91768676847837832132661525
- `0x004B_E8C5_F0B2_BC7E_4A77_9115`
- Ninety-one septillion seven hundred sixty-eight sextillion six hundred
  seventy-six quintillion eight hundred forty-seven quadrillion eight
  hundred thirty-seven trillion eight hundred thirty-two billion one hundred
  thirty-two million six hundred sixty-one thousand five hundred twenty-five

The product computed above requires storage larger than `u64`, and Rust
v1.26 added `u128` type.

Accommodating much longer words and much larger products than this, a
"big num" library is used.
(See [Background](#background) section for details.)
