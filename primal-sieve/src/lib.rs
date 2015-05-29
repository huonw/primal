#![cfg_attr(all(test, feature = "unstable"), feature(test))]

#[cfg(all(test, feature = "unstable"))] extern crate test;
extern crate primal_bit;
extern crate primal_smallsieve;
extern crate primal_estimate;

mod streaming;
pub use streaming::StreamingSieve;

mod sieve;

pub use sieve::Sieve;
