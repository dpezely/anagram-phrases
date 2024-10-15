//! Write results as CSV file: common separated values.

use csv::WriterBuilder;
use std::path::PathBuf;

use crate::error::Result;

/// Persist transpositions and anagrams as JSON file.
pub fn write(
    filepath: &PathBuf, max: usize, singles: &[String], phrases: &[Vec<Vec<String>>],
) -> Result<()> {
    let empty_row: Vec<&str> = vec![];

    let mut f = WriterBuilder::new().flexible(true).from_path(filepath)?;

    f.write_record(["Transpositions"])?;
    for transposition in singles {
        f.write_record([transposition])?;
    }

    f.write_record(&empty_row)?;

    f.write_record(["Anagrams"])?;
    f.write_record(&empty_row)?;
    let limit = phrases.len();
    let mut count = 0;
    for n in 2..=max {
        f.write_record(&[format!("{n} words")])?;
        f.write_record(&empty_row)?;
        for terms in phrases {
            if terms.len() == n {
                let record: Vec<String> = terms.iter().map(|x| x.join("|")).collect();
                f.write_record(record)?;
                count += 1;
            }
        }
        if count == limit {
            break;
        }
        f.write_record(&empty_row)?;
    }

    Ok(())
}
