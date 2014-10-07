//! Simplistic and relatively unoptimised handling of basic
//! tasks around primes:
//!
//! - checking for primality
//! - enumerating primes
//! - factorising numbers
//! - estimating upper and lower bounds for π(*n*) (the number of primes
//!   below *n*) and *p<sub>k</sub>* (the <i>k</i>th prime)
//!
//! This uses a basic Sieve of Eratosthenes to enumerate the primes up to
//! some fixed bound (in a relatively memory efficient manner), and then
//! allows this cached information to be used for things like enumerating
//! the primes, and factorisation via trial division.
//!
//! (Despite the name, it can sieve the primes up to 10<sup>9</sup> in
//! about 5 seconds.)
//!
//! [*Source*](http://github.com/huonw/slow_primes)
//!
//! # Example
//!
//! Let's find the 10001st prime. The basic idea is to enumerate the
//! primes and then take the 10001st in that list.
//!
//! Unfortunately, `Primes::sieve` takes an upper bound, and it gives
//! us no information beyond this; so we really need some way to find
//! an upper bound to be guaranteed to include the 10001st prime. If
//! we had an a priori number we could just use that, but we don't
//! (for the purposes of this example, anyway). Hence, we can either
//! try filtering with exponentially larger upper bounds until we find
//! one that works (e.g. doubling each time), or just take a shortcut
//! and use deeper mathematics via
//! [`estimate_nth_prime`](fn.estimate_nth_prime.html).
//!
//! ```rust
//! // find our upper bound
//! let (_lo, hi) = slow_primes::estimate_nth_prime(10001);
//!
//! // find the primes up to this upper bound
//! let sieve = slow_primes::Primes::sieve(hi as uint);
//!
//! // (.nth is zero indexed.)
//! match sieve.primes().nth(10001 - 1) {
//!     Some(p) => println!("The 10001st prime is {}", p), // 104743
//!     None => unreachable!(),
//! }
//! ```
//!
//! # Using this library
//!
//! Just add the following to your [`Cargo.toml`](http://crates.io/):
//!
//! ```toml
//! [dependencies.slow_primes]
//! git = "https://github.com/huonw/slow_primes"
//! ```

extern crate "num" as num_;

#[cfg(test)] extern crate test;

use num_::Integer;
use std::num;

pub use sieve::{Primes, PrimeIterator};
pub use estimate::{estimate_prime_pi, estimate_nth_prime};

mod estimate;
mod sieve;

#[allow(dead_code)]
mod tables;

/// (prime, exponent) pairs storing the prime factorisation of a
/// number.
pub type Factors = Vec<(uint, uint)>;


fn mod_exp(mut x: u64, mut d: u64, n: u64) -> u64 {
    let mut ret = 1;
    while d != 0 {
        if d % 2 == 1 {
            ret *= x;
            if ret >= n {
                ret %= n;
            }
        }
        d /= 2;
        x *= x;
        if x >= n {
            x %= n;
        }
    }
    ret
}

/// Test if `n` is prime, using the deterministic version of the
/// Miller-Rabin test.
///
/// Doing a lot of primality tests with numbers strictly below some
/// upper bound will be faster using the `is_prime` method of a
/// `Primes` instance.
pub fn is_prime_miller_rabin(n: u64) -> bool {
    static HINT: &'static [u64] = &[2];

    // we have a strict upper bound, so we can just use the witness
    // table of Pomerance, Selfridge & Wagstaff and Jeaschke to be as
    // efficient as possible, without having to fall back to
    // randomness.
    static WITNESSES: &'static [(u64, &'static [u64])] =
        [(2_046, HINT),
         (1_373_652, &[2, 3]),
         (9_080_190, &[31, 73]),
         (25_326_000, &[2, 3, 5]),
         (4_759_123_140, &[2, 7, 61]),
         (1_112_004_669_632, &[2, 13, 23, 1662803]),
         (2_152_302_898_746, &[2, 3, 5, 7, 11]),
         (3_474_749_660_382, &[2, 3, 5, 7, 11, 13]),
         (341_550_071_728_320, &[2, 3, 5, 7, 11, 13, 17]),
         (0xFFFF_FFFF_FFFF_FFFF, &[2, 3, 5, 7, 11, 13, 17, 19, 23])
         ];

    if n % 2 == 0 { return n == 2 }
    if n == 1 { return false }

    let mut d = n - 1;
    let mut s = 0u;
    while d % 2 == 0 { d /= 2; s += 1 }

    let witnesses =
        WITNESSES.iter().find(|&&(hi, _)| hi >= n)
            .map(|&(_, wtnss)| wtnss).unwrap();
    'next_witness: for &a in witnesses.iter() {
        let mut power = mod_exp(a, d, n);
        if power == 1 { continue 'next_witness }

        for _r in range(0, s) {
            if power == n - 1 {
                continue 'next_witness
            }
            power *= power;
            if power >= n {
                power %= n;
            }
        }
        return false
    }

    true
}

/// Returns integers `(y, k)` such that `x = y^k` with `k` maximised
/// (other than for `x = 0, 1`, in which case `y = x`, `k = 1`).
///
/// # Examples
///
/// ```rust
/// assert_eq!(slow_primes::as_perfect_power(2), (2, 1));
/// assert_eq!(slow_primes::as_perfect_power(4), (2, 2));
/// assert_eq!(slow_primes::as_perfect_power(8), (2, 3));
/// assert_eq!(slow_primes::as_perfect_power(1024), (2, 10));
///
/// assert_eq!(slow_primes::as_perfect_power(1000), (10, 3));
///
/// assert_eq!(slow_primes::as_perfect_power(15), (15, 1));
/// ```
pub fn as_perfect_power(x: u64) -> (u64, u8) {
    if x == 0 || x == 1 {
        return (x, 1)
    }

    let floor_log_2 = 64 - x.leading_zeros() - 1;

    let x_ = x as f64;
    let mut last = (x, 1);
    // TODO: we could be smarter about this: we know all the possible
    // primes that can divide the exponent (since we have a list up to
    // 251 >= 64), so we really only need to check them.
    let mut expn = 2u;
    let mut step = 1u;
    while expn <= floor_log_2 {
        let factor = x_.powf(1.0/expn as f64).round() as u64;

        if num::pow(factor, expn) == x {
            last = (factor, expn as u8);
            // if x is a 2nd and 5th power, it's going to be a 10th
            // power too, meaning we can search faster.
            // TODO: check if this is actually saving work
            step = step.lcm(&expn);
        }

        expn += step;
    }
    last
}

/// Return `Some((p, k))` if `x = p^k` for some prime `p` and `k >= 1`
/// (that is, including when `x` is itself a prime).
///
/// Returns `None` if `x` not a perfect power.
pub fn as_prime_power(x: u64) -> Option<(u64, u8)> {
    let (y, k) = as_perfect_power(x);
    if is_prime_miller_rabin(y) {
        Some((y, k))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use std::num;
    use std::iter::range_step;
    use super::{Primes, is_prime_miller_rabin, as_perfect_power, as_prime_power};
    use self::test::Bencher;

    #[test]
    fn miller_rabin() {
        static LIMIT: uint = 1_000_000;
        let sieve = Primes::sieve(LIMIT);
        for x in range(0, LIMIT) {
            let s = sieve.is_prime(x);
            let mr = is_prime_miller_rabin(x as u64);

            assert!(s == mr, "miller_rabin {} mismatches sieve {} for {}",
                    mr, s, x)
        }
    }

    #[test]
    fn perfect_and_prime_power() {
        let tests = [
            (0, (0, 1), false),
            (1, (1, 1), false),
            (2, (2, 1), true),
            (3, (3, 1), true),
            (4, (2, 2), true),
            (5, (5, 1), true),
            (6, (6, 1), false),
            (8, (2, 3), true),
            (9, (3, 2), true),
            (16, (2, 4), true),
            (25, (5, 2), true),
            (32, (2, 5), true),
            (36, (6, 2), false),
            (100, (10, 2), false),
            (1000, (10, 3), false),
                ];

        for &(x, expected, is_prime) in tests.iter() {
            assert_eq!(as_perfect_power(x), expected);
            assert_eq!(as_prime_power(x),
                       if is_prime { Some((expected))} else { None })
        }

        let sieve = Primes::sieve(200);
        let mut primes = sieve.primes();
        static MAX: f64 = 0xFFFF_FFFF_FFFF_FFFFu64 as f64;
        // test a whole pile of (semi)primes
        loop {
            let p = match primes.next() {
                Some(p) => p as u64,
                None => break
            };

            let subprimes = primes.map(|x| (x, false));
            // include 1 to test p itself.
            for (q, is_prime) in Some((1, true)).into_iter().chain(subprimes) {
                let pq = p * q as u64;
                for n in range(1, MAX.log(pq as f64) as uint) {
                    let x = num::pow(pq, n);

                    let expected = (pq, n as u8);
                    assert_eq!(as_perfect_power(x), expected);
                    assert_eq!(as_prime_power(x),
                               if is_prime { Some(expected) } else { None });
                }
            }
        }
    }


    static N: uint = 1_000_000;
    static STEP: uint = 101;
    #[bench]
    fn bench_miller_rabin_tests(b: &mut Bencher) {
        b.iter(|| {
            range_step(1, N, STEP)
                .filter(|&n| is_prime_miller_rabin(n as u64)).count()
        })
    }
    #[bench]
    fn bench_sieve_tests(b: &mut Bencher) {
        b.iter(|| {
            let sieve = Primes::sieve(1_000_000);
            range_step(1, N, STEP)
                .filter(|&n| sieve.is_prime(n)).count()
        })
    }
}
