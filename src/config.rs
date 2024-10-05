//! Configuration (excluding query)

use clap::{Parser, ValueEnum};
use std::convert::From;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::languages::{Encoding, Language};

/// Where to look for dictionary/lexicon files supplied by OS distribution
/// such as those compatible with ispell or GNU aspell.
static DEFAULT_DICT_FILES: LazyLock<Vec<PathBuf>> =
    LazyLock::new(|| vec![PathBuf::from("/usr/share/dict/words")]);

/// Per-instance configuration
///
/// The CLI app uses one instance of this, while the HTTP service uses
/// one per natural language supported.
// For augmenting this struct with CLI args, see:
// https://docs.rs/clap/latest/clap/_derive/index.html#mixing-builder-and-derive-apis
#[derive(Debug, Default, Parser)]
#[clap(max_term_width = 80)]
pub struct Config {
    /// Specify 2 letter ISO code for natural language such as EN for
    /// English, FR for Français, etc. to enable specific filters.
    #[clap(
        short = 'l',
        long = "lang",
        required = false,
        default_value = "EN",
        ignore_case = true
    )]
    pub lang: CliLanguage,

    /// Dictionary files containing one word per line as plain-text.
    /// Files suitable for `ispell` or GNU `aspell` are compatible.
    #[clap(short='d', long="dict", name = "PATH",
           default_values=DEFAULT_DICT_FILES.iter().map(|p| p.as_os_str()))]
    pub dict_file_paths: Vec<PathBuf>,

    /// Specify dictionary encoding; currently only UTF-8, ISO-8859-1
    #[clap(
        short = 'e',
        long = "encoding",
        ignore_case = true,
        name = "X",
        default_value = "UTF_8"
    )]
    pub encoding: CliEncoding,

    /// Defaults to one more than number of words within input phrase
    /// and a minimum of 3 words.
    #[clap(short = 'm', long = "max", default_value = "0", name = "N")]
    pub max_phrase_words: usize,

    /// Include dictionary words containing single letters, which may
    /// help avoid noisy results.  However, specify `--lang` allowing
    /// exceptions of `a` for English, `y` for Spanish, etc.
    // v1.0: name changed and value inverted since v0.4.0 `Options`
    #[clap(short = 's', long = "short")]
    pub include_short: bool,

    /// Include dictionary words containing uppercase, which indicates
    /// being a proper names.  However, specify `--lang` to allow "I" as
    /// an exception for English; etc.
    // v1.0: name changed and value inverted since v0.4.0 `Options`
    #[clap(short = 'u', long = "upcase")]
    pub include_upcase: bool,
}

// Adding clap::ValueEnum to language::Language and language::Encoding
// smelled like a leaky abstration because that's part of our library,
// which shouldn't need to use `clap`.  Therefore, CliLanguage and
// CliEncoding exist as type aliases here.

type CliLanguage = Language;
type CliEncoding = Encoding;

impl ValueEnum for CliLanguage {
    fn value_variants<'a>() -> &'a [Self] {
        &[Language::Any, Language::EN, Language::ES, Language::FR]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let value = match self {
            Language::Any => clap::builder::PossibleValue::new("Any"),
            Language::EN => clap::builder::PossibleValue::new("EN"),
            Language::ES => clap::builder::PossibleValue::new("ES"),
            Language::FR => clap::builder::PossibleValue::new("FR"),
        };
        Some(value)
    }
}

impl ValueEnum for CliEncoding {
    fn value_variants<'a>() -> &'a [Self] {
        &[Encoding::Utf_8, Encoding::Iso_8859_1]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let value = match self {
            Encoding::Utf_8 => clap::builder::PossibleValue::new("utf_8"),
            Encoding::Iso_8859_1 => clap::builder::PossibleValue::new("iso_8859_1"),
        };
        Some(value)
    }
}
