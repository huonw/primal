//! Check some primality-related properties of numbers.
//!
//! This crate is designed to be used via `primal`.

#![no_std]

pub use crate::is_prime::miller_rabin;
pub use crate::perfect_power::{as_perfect_power, as_prime_power};

mod is_prime;
mod perfect_power;
