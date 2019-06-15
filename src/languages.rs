//! Facilitate reasonable setting per natural language.
//!
//! When resolving an anagram, there can be much noise within the
//! results.  This noise may be reduced using simple heuristics like
//! avoiding single letter entries from the dictionary word list.
//! However, there are exceptions where certain single words should be
//! kept.
//!
//! Common single letter words are "a" and "I", but those are specific
//! to English.  Spanish uses "y" as a conjunction.  And so on.
//!
//! This module provides relatively simple mappings for reasonable
//! defaults for each language supported.

use clap::arg_enum;
use std::collections::BTreeMap;

arg_enum! {
    /// Languages currently supported to varying degrees... Pull requests welcome
    #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
    pub enum Available {
        Any,
        // Only necessary if something has been added to `UPCASE` or `SHORT`.
        // Please keep this list sorted alphabetically.
        EN,                         // English; Latin-1
        ES,                         // Spanish, Español; Latin-1
        FR,                         // French, Français; Latin-1
    }
}

impl Default for Available {
    fn default() -> Available {
        Available::Any
    }
}

lazy_static! {    
    /// Associate what words are acceptable when otherwise bypassing
    /// words containing upper case letters.  e.g., "I" isn't a proper
    /// name in English, so it should be allowed.
    /// Anything else should be rejected when the filter is applied to
    /// minimize noise within results.
    pub static ref UPCASE: BTreeMap<Available, Vec<&'static str>> = {
        use Available::*;
        let mut tree = BTreeMap::new();
        tree.insert(EN, vec!["I"]);
        tree.insert(ES, vec![]);
        // Adding an entry here?  Please also add to `SHORT` even if empty
        tree
    };

    /// Associate what words are acceptable when otherwise bypassing
    /// short word.  For instance, "a" is short but should be allowed
    /// for English, and "y" is a conjunction in Spanish.
    /// Anything else should be rejected when the filter is applied to
    /// minimize noise within results.
    pub static ref SHORT: BTreeMap<Available, Vec<&'static str>> = {
        use Available::*;
        let mut tree = BTreeMap::new();
        tree.insert(EN, vec!["I", "a"]);
        tree.insert(ES, vec!["y"]);
        // Adding an entry here?  Please also add to `UPCASE` even if empty
        tree
    };
}
