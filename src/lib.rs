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

use std::{iter, cmp};
use std::collections::{Bitv, bitv};

#[allow(dead_code)]
mod tables;

/// Stores information about primes up to some limit.
///
/// This uses at least `limit / 16 + O(1)` bytes of storage.
pub struct Primes {
    // This only stores odd numbers, since even numbers are mostly
    // non-prime.
    v: Bitv
}

/// Iterator over the primes stored in a sieve.
pub struct PrimeIterator<'a> {
    two: bool,
    iter: iter::Enumerate<bitv::Bits<'a>>,
}

/// (prime, exponent) pairs storing the prime factorisation of a
/// number.
pub type Factors = Vec<(uint, uint)>;

impl Primes {
    /// Construct a `Primes` via a sieve up to at least `limit`.
    ///
    /// This stores all primes less than `limit` (and possibly some
    /// more), allowing for very efficient iteration and primality
    /// testing below this, and guarantees that all numbers up to
    /// `limit^2` can be factorised.
    pub fn sieve(limit: uint) -> Primes {
        // having this out-of-line like this is faster (130 us/iter
        // vs. 111 us/iter on sieve_large), and using a manual while
        // rather than a `range_step` is a similar speedup.
        #[inline(never)]
        fn filter(is_prime: &mut Bitv, limit: uint, check: uint, p: uint) {
            let mut zero = 2 * check * (check + 1);
            while zero < limit / 2 {
                is_prime.set(zero, false);
                zero += p;
            }
        }

        // bad stuff happens for very small bounds.
        let limit = cmp::max(10, limit);

        let mut is_prime = Bitv::with_capacity((limit + 1) / 2, true);
        // 1 isn't prime
        is_prime.set(0, false);

        // multiples of 3 aren't prime (3 is handled separately, so
        // the ticking works properly)
        filter(&mut is_prime, limit, 1, 3);

        let bound = (limit as f64).sqrt() as uint + 1;
        // skip 2.
        let mut check = 2;
        let mut tick = if check % 3 == 1 {2} else {1};

        while check <= bound {
            if is_prime[check] {
                filter(&mut is_prime, limit, check, 2 * check + 1)
            }

            check += tick;
            tick = 3 - tick;
        }

        Primes { v: is_prime }
    }

    /// The largest number stored.
    pub fn upper_bound(&self) -> uint {
        (self.v.len() - 1) * 2 + 1
    }

    /// Check if `n` is prime, possibly failing if `n` is larger than
    /// the upper bound of this Primes instance.
    pub fn is_prime(&self, n: uint) -> bool {
        if n % 2 == 0 {
            // 2 is the evenest prime.
            n == 2
        } else {
            assert!(n <= self.upper_bound());
            self.v[n / 2]
        }
    }

    /// Iterator over the primes stored in this map.
    pub fn primes<'a>(&'a self) -> PrimeIterator<'a> {
        PrimeIterator {
            two: true,
            iter: self.v.iter().enumerate()
        }
    }

    /// Factorise `n` into (prime, exponent) pairs.
    ///
    /// Returns `Err((leftover, partial factorisation))` if `n` cannot
    /// be fully factored, or if `n` is zero (`leftover == 0`). A
    /// number can not be completely factored if and only if the prime
    /// factors of `n` are too large for this sieve, that is, if there
    /// is
    ///
    /// - a prime factor larger than `U^2`, or
    /// - more than one prime factor between `U` and `U^2`
    ///
    /// where `U` is the upper bound of the primes stored in this
    /// sieve.
    ///
    /// Notably, any number between `U` and `U^2` can always be fully
    /// factored, since these numbers are guaranteed to only have zero
    /// or one prime factors larger than `U`.
    pub fn factor(&self, mut n: uint) -> Result<Factors, (uint, Factors)> {
        if n == 0 { return Err((0, vec![])) }

        let mut ret = Vec::new();

        for p in self.primes() {
            if n == 1 { break }

            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                ret.push((p,count));
            }
        }
        if n != 1 {
            let b = self.upper_bound();
            if b * b >= n {
                // n is not divisible by anything from 1...sqrt(n), so
                // must be prime itself! (That is, even though we
                // don't know this prime specifically, we can infer
                // that it must be prime.)
                ret.push((n, 1));
            } else {
                // large factors :(
                return Err((n, ret))
            }
        }
        Ok(ret)
    }
}

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

impl<'a> Iterator<uint> for PrimeIterator<'a> {
    #[inline]
    fn next(&mut self) -> Option<uint> {
        if self.two {
            self.two = false;
            Some(2)
        } else {
            for (i, is_prime) in self.iter {
                if is_prime {
                    return Some(2 * i + 1)
                }
            }
            None
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        let mut iter = *self;
        // TODO: this doesn't run in constant time, is it super-bad?
        match (iter.next(), iter.next_back()) {
            (Some(lo), Some(hi)) => {
                let (below_hi, above_hi) = estimate_prime_pi(hi as u64);
                let (below_lo, above_lo) = estimate_prime_pi(lo as u64);

                ((below_hi - cmp::min(above_lo, below_hi)) as uint,
                 Some((above_hi - below_lo + 1) as uint))
            }
            (Some(_), None) => (1, Some(1)),
            (None, _) => (0, Some(0))
        }
    }
}

impl<'a> DoubleEndedIterator<uint> for PrimeIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<uint> {
        loop {
            match self.iter.next_back() {
                Some((i, true)) => return Some(2 * i + 1),
                Some((_, false)) => {/* continue */}
                None if self.two => {
                    self.two = false;
                    return Some(2)
                }
                None => return None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use std::num;
    use std::iter::range_step;
    use super::{Primes, estimate_prime_pi, estimate_nth_prime,
                is_prime_miller_rabin};
    use self::test::Bencher;

    #[test]
    fn is_prime() {
        let primes = Primes::sieve(1000);
        let tests = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, true)
                ];

        for &(n, expected) in tests.iter() {
            assert_eq!(primes.is_prime(n), expected);
        }
    }

    #[test]
    fn upper_bound() {
        let primes = Primes::sieve(30);
        assert_eq!(primes.upper_bound(), 29);
        let primes = Primes::sieve(31);
        assert_eq!(primes.upper_bound(), 31);

        let primes = Primes::sieve(30000);
        assert_eq!(primes.upper_bound(), 29999);
        let primes = Primes::sieve(30001);
        assert_eq!(primes.upper_bound(), 30001);
    }

    #[test]
    fn primes_iterator() {
        let primes = Primes::sieve(50);
        let mut expected = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        assert_eq!(primes.primes().collect::<Vec<uint>>().as_slice(),
                   expected.as_slice());

        expected.reverse();
        assert_eq!(primes.primes().rev().collect::<Vec<uint>>().as_slice(),
                   expected.as_slice());
    }

    #[test]
    fn factor() {
        let primes = Primes::sieve(1000);

        let tests: &[(uint, &[(uint, uint)])] = &[
            (1, &[]),
            (2, &[(2u, 1)]),
            (3, &[(3, 1)]),
            (4, &[(2, 2)]),
            (5, &[(5, 1)]),
            (6, &[(2, 1), (3, 1)]),
            (7, &[(7, 1)]),
            (8, &[(2, 3)]),
            (9, &[(3, 2)]),
            (10, &[(2, 1), (5, 1)]),

            (2*2*2*2*2 * 3*3*3*3*3, &[(2, 5), (3,5)]),
            (2*3*5*7*11*13*17*19, &[(2,1), (3,1), (5,1), (7,1), (11,1), (13,1), (17,1), (19,1)]),
            // a factor larger than that stored in the map
            (7561, &[(7561, 1)]),
            (2*7561, &[(2, 1), (7561, 1)]),
            (4*5*7561, &[(2, 2), (5,1), (7561, 1)]),
            ];
        for &(n, expected) in tests.iter() {
            assert_eq!(primes.factor(n), Ok(expected.to_vec()));
        }
    }

    #[test]
    fn factor_compare() {
        let short = Primes::sieve(30);
        let long = Primes::sieve(10000);

        let short_lim = short.upper_bound() * short.upper_bound() + 1;
        println!("{}", short_lim)
        // every number less than bound^2 can be factored (since they
        // always have a factor <= bound).
        for n in range(0, short_lim) {
            assert_eq!(short.factor(n), long.factor(n))
        }
        // larger numbers can only sometimes be factored
        'next_n: for n in range(short_lim, 10000) {
            let possible = short.factor(n);
            let real = long.factor(n).unwrap();

            let mut seen_small = None;
            for (this_idx, &(p,i)) in real.iter().enumerate() {
                let last_short_prime = if p >= short_lim {
                    this_idx
                } else if p > short.upper_bound() {
                    match seen_small {
                        Some(idx) => idx,
                        None if i > 1 => this_idx,
                        None => {
                            // we can cope with one
                            seen_small = Some(this_idx);
                            continue
                        }
                    }
                } else {
                    // small enough
                    continue
                };

                // break into the two parts
                let (low, hi) = real.as_slice().split_at(last_short_prime);
                let leftover = hi.iter().fold(1, |x, &(p, i)| x * num::pow(p, i));

                assert_eq!(possible, Err((leftover, low.to_vec())));
                continue 'next_n;
            }

            // if we're here, we know that everything should match
            assert_eq!(possible, Ok(real))
        }
    }

    #[test]
    fn factor_failures() {
        let primes = Primes::sieve(30);

        assert_eq!(primes.factor(0),
                   Err((0, vec![])));
        // can only handle one large factor
        assert_eq!(primes.factor(31 * 31),
                   Err((31 * 31, vec![])));
        assert_eq!(primes.factor(2 * 3 * 31 * 31),
                   Err((31 * 31, vec![(2, 1), (3, 1)])));

        // prime that's too large (bigger than 30*30).
        assert_eq!(primes.factor(7561),
                   Err((7561, vec![])));
        assert_eq!(primes.factor(2 * 3 * 7561),
                   Err((7561, vec![(2, 1), (3, 1)])));
    }

    #[test]
    fn size_hint() {
        for i in range_step(0, 1000, 100) {
            let sieve = Primes::sieve(i);

            let mut primes = sieve.primes();

            // check the size hint at each and every iteration
            loop {
                let (lo, hi) = primes.size_hint();

                let mut copy = primes;
                let len = copy.count();

                let next = primes.next();

                assert!(lo <= len && len <= hi.unwrap(),
                        "found failing size_hint for {} to {}, should satisfy: {} <= {} <= {}",
                        next, i, lo, len, hi);

                if next.is_none() {
                    break
                }
            }
        }
    }

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

    #[bench]
    fn sieve_small(b: &mut Bencher) {
        b.iter(|| Primes::sieve(100))
    }
    #[bench]
    fn sieve_medium(b: &mut Bencher) {
        b.iter(|| Primes::sieve(10_000))
    }
    #[bench]
    fn sieve_large(b: &mut Bencher) {
        b.iter(|| Primes::sieve(100_000))
    }

    fn bench_iterate(b: &mut Bencher, upto: uint) {
        let sieve = Primes::sieve(upto);

        b.iter(|| {
            sieve.primes().count()
        })
    }

    #[bench]
    fn iterate_small(b: &mut Bencher) { bench_iterate(b, 100) }
    #[bench]
    fn iterate_large(b: &mut Bencher) { bench_iterate(b, 100_000) }


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
