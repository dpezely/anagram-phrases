use num_bigint::{BigUint, ToBigUint};
use std::collections::BTreeMap;

use crate::error::ErrorKind;
use crate::primes::*;

#[test]
fn uniques() {
    assert_eq!(extract_unique_chars("this-is-that"), "thisa".to_string());
    assert_eq!(extract_unique_chars("this is that"), "thisa".to_string());
    assert_eq!(extract_unique_chars("This Is That"), "thisa".to_string());
    assert_eq!(extract_unique_chars("My cat's hat"), "mycatsh".to_string());
    assert_eq!(extract_unique_chars("Liberté, Égalité, Fraternité"),
               "libertégafn".to_string())
}

#[test]
fn essentials() {
    assert_eq!(essential_chars("Foo"), "foo".to_string());
    assert_eq!(essential_chars("foo"), "foo".to_string());
    assert_eq!(essential_chars("aaaaaaa"), "aaaaaaa".to_string());
}

#[test]
fn mismatch() {
    assert!(matched_chars("a", "a"));
    assert!(matched_chars("m", "m"));
    assert!(matched_chars("z", "z"));
    assert!(matched_chars("abc", "abc"));
    assert!(matched_chars("abc", "cba"));
    assert!(matched_chars("cba", "abc"));
    assert!(matched_chars("a", "abc"));
    assert!(matched_chars("aaaaabbbccc", "abc"));
    assert!(matched_chars("bc", "abc"));
    assert!(!matched_chars("abcz", "abc"));
    assert!(!matched_chars("z", "abc"));
}

/// Prime numbers corresponding to each letter of `word`
/// where A=2, B=3, ... Z=101:
#[test]
fn product() {
    #[cfg(not(feature = "disable-u128"))]
    {
        let word = "superconductivity";
        let big: u128 = 67 * 73 * 53 * 11 * 61
            * 5 * 47 * 43 * 7 * 73 * 5 * 71 * 23 * 79 * 23 * 71 * 97;
        let product = primes(word).unwrap();
        assert_eq!(primes_product(&product).unwrap(),
                   big.to_biguint().unwrap());
    }
    #[cfg(feature = "disable-u128")]
    {
        let word = "conductivity";
        let big: u64 =
            5 * 47 * 43 * 7 * 73 * 5 * 71 * 23 * 79 * 23 * 71 * 97;
        let product = primes(word).unwrap();
        assert_eq!(primes_product(&product).unwrap(),
                   big.to_biguint().unwrap());
    }
}

#[test]
fn filtering() {
    let product: BigUint = 2u8.to_biguint().unwrap();
    match filter_word("abc", "a", 1, &product) {
        Err(ErrorKind::WordTooLong) => {}
        other => panic!("expected: {} received: {:?}",
                        ErrorKind::WordTooLong, other)
    }
    match filter_word("z", "a", 1, &product) {
        Err(ErrorKind::MismatchedChars) => {}
        other => panic!("expected: {} received: {:?}",
                        ErrorKind::MismatchedChars, other)
    }

    let product: BigUint = (2 * 3 * 5 * 101).to_biguint().unwrap();
    match filter_word("zzz", "abcz", 4, &product) {
        Err(ErrorKind::WordProductTooBig) => {}
        other => panic!("expected: {} received: {:?}",
                        ErrorKind::WordProductTooBig, other)
    }
}

#[test]
fn positive_1() {
    with_static_dictionary("torchwood", &["doctor", "who"],
                           &[71,47,61,5,19,83,47,47,7]);
}

#[test]
fn positive_2() {
    with_static_dictionary("panic moon", &["companion"],
                           &[53,2,43,23,5,41,47,47,43]);
}

fn with_static_dictionary(input: &str, dictionary: &[&str], primes: &[u16]) {
    let essential = essential_chars(input);
    let input_length = essential.len();
    let pattern = extract_unique_chars(input);
    let product = primes.iter().fold(1, |acc, &x| acc * x as usize);
    match primes_product(primes) {
        Ok(input_product) => {
            assert_eq!(product.to_biguint().unwrap(), input_product);
            let mut map: Map = BTreeMap::new();
            let mut wordlist: Vec<String> = vec![];
            for word in dictionary {
                if let Ok(product) = filter_word(word, &pattern, input_length,
                                                 &input_product) {
                    map.entry(product)
                        .or_insert(Vec::with_capacity(1))
                        .push(word.to_string());
                    wordlist.push(word.to_string());
                }
            }
            assert_eq!(map.len(), dictionary.len());
            assert_eq!(wordlist, dictionary)
        }
        other =>
            panic!("expected: {} received: {:?}", product, other)
    }
}

#[cfg(not(feature="external-hasher"))]
#[test]
fn latin1() {
    // Within the ASCII range, accept only [a-z]
    assert!(hash('\u{0060}').is_none());
    assert!(hash('\u{0061}').is_some());  // `a`
    assert!(hash('\u{006f}').is_some());  // `o`
    assert!(hash('\u{007a}').is_some());  // `z`
    assert!(hash('\u{007b}').is_none());

    assert!(hash('\u{00A0}').is_none()); // NBSP

    // ISO-8859-1 has à=U+00E0 for lowercase
    assert!(hash('\u{00e0}').is_some());
    // ISO-8859-1 has ÿ=U+00FF for lowercase
    assert!(hash('\u{00ff}').is_some());
}

#[cfg(feature="external-hasher")]
#[test]
fn latin_extended_a() {
    // Within the ASCII range, accept only [a-z]
    assert!(hash('\u{0060}').is_none());
    assert!(hash('\u{0061}').is_some());  // `a`
    assert!(hash('\u{006f}').is_some());  // `o`
    assert!(hash('\u{007a}').is_some());  // `z`
    assert!(hash('\u{007b}').is_none());

    assert!(hash('\u{00a0}').is_none()); // NBSP

    // https://en.wikipedia.org/wiki/Latin_Extended-A
    // ā 	Latin Small letter A with macron
    assert!(hash('\u{0101}').is_some());
    // https://en.wikipedia.org/wiki/Latin_Extended-B
    // ƀ 	Latin Small Letter B with Stroke
    assert!(hash('\u{0180}').is_some());
}
