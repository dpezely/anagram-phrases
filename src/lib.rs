#[macro_use]
extern crate lazy_static;
extern crate num_bigint;
extern crate num_traits;

#[cfg(feature="external-hasher")]
extern crate char_seq;

pub mod error;
pub mod languages;
pub mod primes;
pub mod search;
pub mod session;
#[cfg(test)]
pub mod test_languages;
#[cfg(test)]
pub mod test_primes;
