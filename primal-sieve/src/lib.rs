#![cfg_attr(all(test, feature = "unstable"), feature(test))]

#[cfg(all(test, feature = "unstable"))] extern crate test;
extern crate primal_bit;
extern crate primal_smallsieve;
extern crate primal_estimate;
extern crate hamming;

mod streaming;
pub use streaming::StreamingSieve;

// mod wheel6;
#[macro_use]
mod wheel {
    #[derive(Debug)]
    pub struct WheelInfo {
        pub true_prime: usize,
        pub prime: usize,
        pub wheel_index: usize,
        pub sieve_index: usize,
    }
    #[derive(Debug)]
    pub struct WheelInit {
        pub next_mult_factor: u8,
        pub wheel_index: u8,
    }
    #[macro_escape]
    macro_rules! init {
        ($nmf: expr, $wi: expr) => { ::wheel::WheelInit { next_mult_factor: $nmf, wheel_index: $wi }}
    }
    #[derive(Debug)]
    pub struct WheelElem {
        pub unset_bit: u8,
        pub next_mult_factor: u8,
        pub correction: u8,
        pub next: i8,
    }
    #[macro_escape]
    macro_rules! elem {
        ($bit: expr, $nmf: expr, $c: expr, $n: expr) => {
            ::wheel::WheelElem {
                unset_bit: 1u8 << $bit,
                next_mult_factor: $nmf,
                correction: $c,
                next: $n,
            }
        }
    }
    pub use wheel210::{bit_index, from_bit_index, set_bit, compute_wheel_elem, MODULO, SIZE};
}
mod wheel30;
mod wheel210;
mod sieve;

pub use sieve::Sieve;
