//! Highly optimised prime sieves.
//!
//! This is designed to be used via the `primal` crate.

#![cfg_attr(feature = "no-std", no_std)]

// black boxes for pointers; LLVM isn't so happy without
// them. Unfortunately only usable with 1.59+ asm!, but the code isn't
// *too* much slower without them.
#[cfg(feature = "unstable")]
#[inline(always)]
fn b<T>(mut p: *mut T) -> *mut T { unsafe { core::arch::asm!("/* {0} */", inout(reg) p) } p }
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
