//! Check some primality-related properties of numbers.
//!
//! This crate is designed to be used via `primal`.
extern crate num;
#[cfg(test)]
extern crate primal;

pub use is_prime::miller_rabin;
pub use perfect_power::{as_perfect_power, as_prime_power};

mod perfect_power;
mod is_prime;
