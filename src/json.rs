//! Write results as JSON file.

use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::error::Result;

/// Organization within JSON file.
///
/// Isolate single words ("transpositions") from multi-word phrases
/// ("anagrams").
#[derive(Serialize)]
struct JsonExport<'a, 'b> {
    /// "Transpositions" are results strictly consisting of single words.
    transpositions: &'a [String],
    /// "Anagrams" are results strictly consisting of multiple words.
    anagrams: &'b [Vec<Vec<String>>],
}

/// Persist transpositions and anagrams as JSON file.
pub fn write(
    filepath: &PathBuf, max: usize, singles: &[String], phrases: &[Vec<Vec<String>>],
) -> Result<()> {
    let mut f = File::create(filepath)?;

    let mut anagrams: Vec<Vec<Vec<String>>> = Vec::with_capacity(phrases.len());
    let limit = phrases.len();
    let mut count = 0;
    for n in 2..=max {
        for terms in phrases {
            if terms.len() == n {
                anagrams.push(terms.clone());
                count += 1;
            }
        }
        if count == limit {
            break;
        }
    }

    let export = JsonExport { transpositions: singles, anagrams: &anagrams };
    f.write_all(serde_json::to_string(&export)?.as_bytes())?;
    Ok(())
}
