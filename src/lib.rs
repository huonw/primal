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

#[allow(dead_code)]
mod tables;

mod sieve;

/// (prime, exponent) pairs storing the prime factorisation of a
/// number.
pub type Factors = Vec<(uint, uint)>;

/// Returns estimated bounds for π(*n*), the number of primes less
/// than or equal to `n`.
///
/// That is, if (*a*, *b*) = `estimate_prime_pi(n)`, *a* ≤ π(*n*) ≤
/// *b*. The bounds used are proved in [1] and [2, Théorème 1.10],
/// and are summarised in [2, pp. 14–15].
///
/// [1]: Barkley Rosser. "Explicit Bounds for Some Functions of Prime
/// Numbers". American Journal of Mathematics 63 (1):
/// 211–232. 1941. doi:[10.2307/2371291](http://dx.doi.org/10.2307/2371291).
///
///  [2]: Dusart, Pierre. ["Autour de la fonction qui compte le nombre
/// de nombres premiers."][pdf] PhD diss., Université de Limoges,
/// 1998.
///
/// [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
pub fn estimate_prime_pi(n: u64) -> (u64, u64) {
    if n < tables::SMALL_PRIME_PI.len() as u64 {
        let x = tables::SMALL_PRIME_PI[n as uint] as u64;
        (x, x)
    } else {
        let n_ = n as f64;
        let lg = n_.ln();
        let inv_lg = 1.0 / lg;
        let n_lg = n_ * inv_lg;

        // numbers refer to parts of theorem 1.10 of [2].
        let lo = if n >= 32299 {
            // 6.
            n_lg * (1.0 + inv_lg * (1.0 + 1.8 * inv_lg))
        } else if n >= 5393 {
            // 5.
            n_ / (lg - 1.0)
        } else if n >= 599 {
            // 1.
            n_lg * (1.0 + inv_lg)
        } else {
            // [1]
            n_ / (lg + 2.0)
        };

        let hi = if n >= 13_220_000_000 {
            // 3.
            n_lg * (1.0 + 1.0992 * inv_lg)
        } else if n >= 355991 {
            // 7.
            n_lg * (1.0 + inv_lg * (1.0 + 2.51 * inv_lg))
        } else if n >= 60184 {
            // 4.
            n_ / (lg - 1.1)
        } else {
            // 2.
            n_lg * (1.0 + 1.2762 * inv_lg)
        };

        (lo as u64, hi as u64)
    }
}

/// Gives estimated bounds for *p<sub>n</sub>*, the `n`th prime number,
/// 1-indexed (i.e. *p<sub>1</sub>* = 2, *p<sub>2</sub>* = 3).
///
/// That is, if (<i>a</i>,<i>b</i>) = `estimate_nth_prime(n)`, *a* ≤
/// *p<sub>n</sub>* ≤ *b*. The bounds used are proved in [1] and [2,
/// Théorèmes 1.6–1.8], and are summarised in [2, pp. 14–15].
///
/// [1]: Massias, Jean-Pierre; Robin, Guy. ["Bornes effectives pour
/// certaines fonctions concernant les nombres
/// premiers."](http://eudml.org/doc/247826) Journal de théorie des
/// nombres de Bordeaux 8.1 (1996): 215-242.
///
/// [2]: Dusart, Pierre. ["Autour de la fonction qui compte le nombre
/// de nombres premiers."][pdf] PhD diss., Université de Limoges, 1998.
///
/// [pdf]: http://www.unilim.fr/laco/theses/1998/T1998_01.html
pub fn estimate_nth_prime(n: u64) -> (u64, u64) {
    if n == 0 {
        (0, 0)
    } else if n <= tables::SMALL_PRIMES.len() as u64 {
        // table is 0-indexed, n is 1-indexed, need to adjust.
        let x = tables::SMALL_PRIMES[n as uint - 1] as u64;
        (x, x)
    } else {
        let n_ = n as f64;
        let lg = n_.ln();
        let lglg = lg.ln();

        let lo = lg + lglg - 1.0 + if n > 3 {//13196 {
            // [2] Theorem 1.6
            (lglg - 2.1) / lg
        } else {
            // [1] Theorem A (ii)
            0.0
        };

        let hi = lg + lglg + if n >= 39017 {
            // [2] Theorem 1.8
            -0.9484
        } else if n >= 27076 {
            // [2] Theorem 1.7
            -1.0 + (lglg - 1.8) / lg
        } else if n >= 15985 {
            // [1] Theorem A (v)
            -0.9427
        } else if n >= 13 {
            // [1] Theorem A (v)
            -1.0 + 1.8 * lglg / lg
        } else {
            // [1] Theorem A (iv)
            0.0
        };
        ((n_ * lo) as u64, (n_ * hi) as u64)
    }
}

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
    use super::{Primes, estimate_prime_pi, estimate_nth_prime,
                is_prime_miller_rabin, as_perfect_power, as_prime_power};
    use self::test::Bencher;

    #[test]
    fn prime_pi() {
        fn check(n: u64, pi: u64) {
            let (lo, hi) = estimate_prime_pi(n);
            assert!(lo <= pi && pi <= hi,
                    "found failing estimate at {}, should satisfy: {} <= {} <= {}",
                    n, lo, pi, hi)
        }
        let primes = Primes::sieve(1_000_000);

        let mut last = 0;
        for (i, p) in primes.primes().enumerate() {
            for j in range(last, p) {
                check(j as u64, i as u64);
            }
            last = p;
        }

        let sporadic = [
            (1, 4),
            (2, 25),
            (3, 168),
            (4, 1229),
            (5, 9592),
            (6, 78498),
            (7, 664579),
            (8, 5761455),
            (9, 50847534),
            (10, 455052511),
            (11, 4118054813),
            (12, 37607912018),
            (13, 346065536839),
            (14, 3204941750802),
            (15, 29844570422669),
            (16, 279238341033925),
            (17, 2623557157654233),
            ];
        for &(exponent, real) in sporadic.iter() {
            let n = num::pow(10, exponent);
            check(n, real);
        }
    }

    #[test]
    fn nth_prime() {
        fn check(n: u64, p: u64) {
            let (lo, hi) = estimate_nth_prime(n);
            assert!(lo <= p && p <= hi,
                    "found failing estimate at {}, should satisfy: {} <= {} <= {}",
                    n, lo, p, hi);
        }
        let sieve = Primes::sieve(1_000_000);

        for (i, p) in sieve.primes().enumerate() {
            let n = i as u64 + 1;
            check(n, p as u64);
        }

        let sporadic = [
            (0, 2),
            (1, 29),
            (2, 541),
            (3, 7919),
            (4, 104729),
            (5, 1299709),
            (6, 15485863),
            (7, 179424673),
            (8, 2038074743),
            (9, 22801763489),
            (10, 252097800623),
            (11, 2760727302517),
            (12, 29996224275833),
            (13, 323780508946331),
            (14, 3475385758524527),
            (15, 37124508045065437),
            ];

        for &(exponent, nth_prime) in sporadic.iter() {
            let n = num::pow(10, exponent);
            check(n, nth_prime);
        }
    }

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
