//! Highly optimised prime sieves.
//!
//! This is designed to be used via the `primal` crate.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate alloc;

// black boxes for pointers; LLVM isn't so happy without
// them. Unfortunately only usable with 1.59+ asm!, but the code isn't
// *too* much slower without them.
#[cfg(feature = "unstable")]
#[inline(always)]
fn b<T>(mut p: *mut T) -> *mut T {
    unsafe { core::arch::asm!("/* {0} */", inout(reg) p) }
    p
}
#[cfg(not(feature = "unstable"))]
#[inline(always)]
fn b<T>(p: *mut T) -> *mut T {
    p
}

#[cfg(feature = "safe")]
macro_rules! safe_assert {
    ($x: expr) => {
        assert!($x);
    };
}
#[cfg(not(feature = "safe"))]
macro_rules! safe_assert {
    ($x: expr) => {
        ()
    };
}

mod streaming;
pub use crate::streaming::primes::Primes;
pub use crate::streaming::StreamingSieve;

// mod wheel6;
mod sieve;
mod wheel;

pub use crate::sieve::{Sieve, SievePrimes};
