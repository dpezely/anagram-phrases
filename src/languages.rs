//! Facilitate reasonable settings per natural language.
//!
//! When resolving an anagram, there can be much noise within the
//! results.  This noise may be reduced using simple heuristics like
//! avoiding single-letter entries from the dictionary word list.
//! However, there are exceptions where certain single-letter words
//! should be kept.
//!
//! Some single-letter words are "a" and "I", but those are specific
//! to English.  Spanish uses "y" as a conjunction.  And so on.
//!
//! This module provides relatively simple mappings for reasonable
//! defaults for each language supported.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::convert::From;
use std::sync::LazyLock;

use crate::error::AnagramError;

/// Languages currently supported to varying degrees... Pull requests welcome
#[derive(Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Language {
    Any,
    // Only necessary if something has been added to `UPCASE` or `SHORT`.
    // Please keep this list sorted alphabetically.
    EN, // English; Latin-1
    ES, // Spanish, Español; Latin-1
    FR, // French, Français; Latin-1
}

/// Region portion of "LANG" environment variable.
///
/// Complements [Language] such that its `EN` for English may be
/// further distinguished between UK, US, Canada, etc.  This
/// corresponds to the second component of LANG environment variable;
/// e.g., in Bash, `export LANG="en-US"`
#[derive(Deserialize, Clone, Debug, Default, Eq, PartialEq)]
pub enum Region {
    #[default]
    Any,
    // Please keep this alphabetized:
    CA,
    GB, // official but some use UK
    UK, // unofficial but often preferred to GB
    US,
}

#[derive(Debug, Default, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Encoding {
    #[default]
    Utf_8,
    Iso_8859_1,
}

/// Words allowed to start with uppercase.
///
/// Associate what words are acceptable when otherwise bypassing
/// words containing upper case letters.  e.g., "I" isn't a proper
/// noun/name in English, so it should be allowed.
/// Anything else should be rejected when the filter is applied to
/// minimize noise within results.
pub static UPCASE: LazyLock<BTreeMap<Language, Vec<&'static str>>> =
    LazyLock::new(|| {
        use Language::*;
        let mut tree = BTreeMap::new();
        tree.insert(EN, vec!["I"]);
        tree
    });

/// Words allowed to be short.
///
/// Associate what words are acceptable when otherwise bypassing
/// short word.  For instance, "a" is short but should be allowed
/// for English, and "y" is a conjunction in Spanish.
/// Anything else should be rejected when the filter is applied to
/// minimize noise within results.
///
/// Values are [str] (rather than [char]) in case a natural language
/// needs to filter more than single character words.
pub static SHORT: LazyLock<BTreeMap<Language, Vec<&'static str>>> = LazyLock::new(|| {
    use Language::*;
    let mut tree = BTreeMap::new();
    tree.insert(EN, vec!["I", "a"]);
    tree.insert(ES, vec!["a", "y"]);
    tree.insert(FR, vec!["a", "d", "i", "j", "l", "m", "s", "t", "y"]);
    tree
});

impl std::str::FromStr for Language {
    type Err = AnagramError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_uppercase().as_str() {
            "ANY" => Ok(Language::Any),
            "EN" => Ok(Language::EN),
            "ES" => Ok(Language::ES),
            "FR" => Ok(Language::FR),
            _ => Err(AnagramError::LangNotImplemented),
        }
    }
}

impl From<&str> for Language {
    fn from(string: &str) -> Language {
        match string.to_uppercase().as_str() {
            "EN" => Language::EN,
            "ES" => Language::ES,
            "FR" => Language::FR,
            _ => Language::Any,
        }
    }
}

impl From<&str> for Region {
    fn from(string: &str) -> Region {
        match string.to_uppercase().as_str() {
            "CA" => Region::CA,
            "GB" => Region::GB,
            "UK" => Region::GB, // UK is ISO-unofficial but regionally correct
            "US" => Region::US,
            _ => Region::Any,
        }
    }
}

/// Language-specific filtering for words from dictionary.
///
/// Supply `SHORT_WORDS` and `UPCASE_WORDS` for `short_words` and
/// `upcase_words`, respectively, for words that are *allowed*.
/// Boolean parameters take precedence over supplied word lists.
/// Return value indicates whether to reject dictionary `word` or not.
#[inline]
pub fn filter(
    word: &str, short_words: &[&str], upcase_words: &[&str], skip_short: bool,
    skip_upcase: bool,
) -> bool {
    if word.len() == 1 {
        if skip_short {
            true
        } else if short_words.is_empty() {
            false
        } else {
            !short_words.iter().any(|&w| w == word)
        }
    } else if let Some(ch) = word.chars().next() {
        if ch.is_uppercase() {
            if skip_upcase {
                true
            } else if upcase_words.is_empty() {
                false
            } else {
                !upcase_words.iter().any(|&w| w == word)
            }
        } else {
            false
        }
    } else {
        false
    }
}

/// Parse `lang` as value of "LANG" environment variable.
///
/// For example, "en_CA.UTF-8" or "en_US".  Case is insignificant.
/// (Pre-processing required if used with value from HTTP header,
/// "Accept-Language" to strip weighting.)
pub fn parse_lang(lang: &str) -> (Language, Region) {
    let mut language = Language::Any;
    let mut region = Region::Any;
    match lang.len() {
        2 => {
            language = Language::from(lang);
        }
        n if n >= 5 => {
            // Character at index=2 may be '_' or '-'
            // FIXME: ignores ending such as ".UTF-8" or ".Latin-2"
            language = Language::from(&lang[0..2]);
            region = Region::from(&lang[3..5]);
        }
        _ => {}
    }
    (language, region)
}
