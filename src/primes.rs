use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::One;
use std::collections::BTreeMap;
use std::ops::Rem;

#[cfg(feature = "external-hasher")]
use char_seq;

use crate::error::{AnagramError, Result};

pub type Map = BTreeMap<BigUint, Vec<String>>;

/// This is a sequence of mathematical prime numbers, whereby each
/// letter of a given alphabet within a script such as Latin or
/// Cyrillic gets uniquely mapped/hashed to one of these.
///
/// Specifically, the hasher function assigns each char to a unique
/// index within this array.  That hasher need only provide unique
/// output per natural language.  (For concurrency use cases, simply
/// isolate hasher results by natural language; e.g., isolate English
/// from Français yet may be mixed for fr_CA and fr_FR, but
/// dictionaries would differ.)
#[rustfmt::skip]
// TODO: per-LANG mapping from UTF-8 to continguous sequence of primes.
//   a,b,c,d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z
const PRIMES: [u16; 200] =
    [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,
     // Additional primes for non-English languages; e.g., ISO-8859-1
     // and Windows-1252 for à U+00E0 through ÿ U+00FF code points:
     103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,
     191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,
     // Expand to include all ISO-8859-* and ranges from UTF-8.
     // Accommodating both modern and historical scripts within a single
     // use case, such as contemporary Cyrillic quoting an ancient passage:
     277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,
     379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,
     467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,577,
     587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,673,
     677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,
     797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,
     907,911,919,929,937,941,947,953,967,971,977,983,991,997,1009,1013,
     1019,1021,1031,1033,1039,1049,1051,1061,1063,1069,1087,1091,1093,
     1097,1103,1109,1117,1123,1129,1151,1153,1163,1171,1181,1187,1193,
     1201,1213,1217,1223];

/// For a given `word` from a natural language dictionary (different
/// from the "input" phrase to be resolved), evaluate that word based
/// upon: `pattern` is the set of unique characters extracted from the
/// input phrase; `input_length` is the string length of the input
/// phrase but counting only alphabetic characters; `input_product` is
/// the mathematical product of multiplying all prime numbers
/// associated with all alphanumeric characters (not just uniques)
/// from the input phrase.
pub fn filter_word(
    word: &str, pattern: &str, input_length: usize, input_product: &BigUint,
) -> Result<BigUint> {
    let word_chars = essential_chars(word);
    if word_chars.len() > input_length {
        return Err(AnagramError::WordTooLong);
    }
    let unique_chars = extract_unique_chars(word);
    if !matched_chars(&unique_chars, pattern) {
        return Err(AnagramError::MismatchedChars);
    }
    let product = primes_product(&primes(&word_chars)?)?;
    if product > *input_product {
        return Err(AnagramError::WordProductTooBig);
    }
    if input_product.rem(&product) != BigUint::ZERO {
        return Err(AnagramError::WordProductNotFactor);
    }
    Ok(product)
}

/// Extract non-duplicate characters in preparation for pattern-matching
#[allow(clippy::map_entry)]
pub fn extract_unique_chars(word: &str) -> String {
    let mut pattern = String::with_capacity(word.len());
    let mut map: BTreeMap<char, bool> = BTreeMap::new();
    for ch in word.to_lowercase().chars() {
        if ch.is_alphabetic() && !map.contains_key(&ch) {
            map.insert(ch, true);
            pattern.push(ch);
        }
    }
    pattern
}

/// Extract alphabetic characters while allowing duplicate characters
/// but ignoring white-space, hyphens, apostrophes, etc.
/// e.g., for determining length used while filtering word list entries
pub fn essential_chars(word: &str) -> String {
    let mut pattern = String::with_capacity(word.len());
    for ch in word.to_lowercase().chars() {
        if ch.is_alphabetic() {
            pattern.push(ch);
        }
    }
    pattern
}

/// Accept dictionary `word` when it contains only chars within `pattern`
pub fn matched_chars(word: &str, pattern: &str) -> bool {
    for ch in word.chars() {
        if pattern.find(ch).is_none() {
            return false;
        }
    }
    true
}

/// Hash a string's "essential" characters to a sequence of prime numbers.
/// See `essential_chars()`.
pub fn primes(essential: &str) -> Result<Vec<u16>> {
    let mut result = Vec::with_capacity(essential.len());
    for ch in essential.chars() {
        if let Some(index) = hash(ch) {
            result.push(PRIMES[index]);
        } else {
            // Probably char_seq::hasher() is incomplete.
            // Perhaps .to_lowercase() didn't work as expected?
            println!(
                "Error: unable to select a prime \
                      for '{}' (U+{:04x}) in \"{}\"",
                ch, ch as usize, &essential
            );
            return Err(AnagramError::CharOutOfBounds);
        }
    }
    Ok(result)
}

/// For a set of prime numbers, multiply all of them together
/// producing a single product.  This result may overflow `u64` or
/// `u128`, so a big num implementation, `num-bigint` crate, is used.
pub fn primes_product(primes: &[u16]) -> Result<BigUint> {
    let mut result = One::one();
    for p in primes.iter() {
        if let Some(bignum) = ToBigUint::to_biguint(p) {
            result *= bignum;
        } else {
            return Err(AnagramError::PrimeTooBig);
        }
    }
    Ok(result)
}

/// Map a char code-point from ISO-8859-* to index within `PRIMES`
#[cfg(not(feature = "external-hasher"))]
#[inline]
pub fn hash(ch: char) -> Option<usize> {
    if ch.is_ascii_lowercase() {
        // Accommodate all of ISO-8859-1 through -16
        // ASCII a=97,U+61, z=122,U+7A; 26 lowercase characters
        Some(ch as usize - 0x61)
    } else if ('\u{00A1}'..='\u{00FF}').contains(&ch) {
        // skip NBSP
        // e.g., ISO-8859-1 has à=U+00E0, ÿ=U+00FF for lowercase
        // Not the most compact for iso-8859-1 but maintains
        // integrity for Cyrillic in iso-8859-5
        Some(26 + ch as usize - 0xA1)
    } else {
        None
    }
}

#[cfg(feature = "external-hasher")]
#[inline]
pub fn hash(ch: char) -> Option<usize> {
    // To replace this external dependency with your own, the
    // Cargo Guide section on Overriding Dependencies might help:
    //https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#overriding-dependencies
    char_seq::hash(ch)
}
