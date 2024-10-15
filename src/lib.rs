extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;

#[cfg(feature = "external-hasher")]
extern crate char_seq;

pub mod config;
#[cfg(feature = "cli")]
pub mod csv;
pub mod error;
#[cfg(feature = "cli")]
pub mod json;
pub mod languages;
pub mod primes;
pub mod search;
#[cfg(test)]
mod test_languages;
#[cfg(test)]
mod test_primes;
#[cfg(test)]
mod test_search;
pub mod words;
