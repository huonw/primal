//! Highly optimised prime sieves.
//!
//! This is designed to be used via the `primal` crate.

#![cfg_attr(feature = "unstable", feature(llvm_asm))]
#![deny(warnings)]

#[cfg(test)] extern crate primal_slowsieve;
extern crate primal_bit;
extern crate primal_estimate;
extern crate smallvec;

// black boxes for pointers; LLVM isn't so happy without
// them. Unfortunately only usable with unstable, but the code isn't
// *too* much slower without them.
#[cfg(feature = "unstable")]
#[inline(always)]
fn b<T>(mut p: *mut T) -> *mut T { unsafe { llvm_asm!("": "+r"(p)) } p }
#[cfg(not(feature = "unstable"))]
#[inline(always)]
fn b<T>(p: *mut T) -> *mut T { p }

#[cfg(feature = "safe")]
macro_rules! safe_assert {
    ($x: expr) => {
        assert!($x);
    }
}
#[cfg(not(feature = "safe"))]
macro_rules! safe_assert {
    ($x: expr) => { () }
}

mod streaming;
pub use crate::streaming::StreamingSieve;
pub use crate::streaming::primes::Primes;

// mod wheel6;
mod wheel;
mod sieve;

pub use crate::sieve::{Sieve, SievePrimes};
