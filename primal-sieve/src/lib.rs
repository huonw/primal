#![cfg_attr(feature = "unstable", feature(asm))]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

#[cfg(all(test, feature = "unstable"))] extern crate test;
#[cfg(test)] extern crate primal_smallsieve;
extern crate primal_bit;
extern crate primal_estimate;
extern crate hamming;

// black boxes for pointers; LLVM isn't so happy without
// them. Unfortunately only usable with unstable, but the code isn't
// *too* much slower without them.
#[cfg(feature = "unstable")]
#[inline(always)]
fn b<T>(mut p: *mut T) -> *mut T { unsafe { asm!("": "+r"(p)) } p }
#[cfg(not(feature = "unstable"))]
#[inline(always)]
fn b<T>(p: *mut T) -> *mut T { p }

#[cfg(feature = "unstable")]
#[inline(always)]
fn b_imm<T>(mut p: *const T) -> *const T { unsafe { asm!("": "+r"(p)) } p }
#[cfg(not(feature = "unstable"))]
#[inline(always)]
fn b_imm<T>(p: *const T) -> *const T { p }

mod streaming;
pub use streaming::StreamingSieve;

// mod wheel6;
mod wheel;
mod sieve;

pub use sieve::Sieve;
