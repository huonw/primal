//! Slow and basic handling of primes.

use std::{iter, cmp};
use std::collections::{Bitv, bitv};

/// Stores information about primes.
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

impl Primes {
    /// Construct a `Primes` via a sieve, stopping at `upto`.
    pub fn sieve(upto: uint) -> Primes {
        // bad stuff happens for very small bounds.
        let upto = cmp::max(10, upto);

        let mut is_prime = Bitv::with_capacity((upto + 1) / 2, true);
        // 1 isn't prime
        is_prime.set(0, false);

        // multiples of 3 aren't prime (3 is handled separately, so
        // the ticking works properly)
        for i in iter::range_step(4, upto / 2, 3) {
            is_prime.set(i, false);
        }

        let mut check = 2; // 5
        let mut tick = 1; // step by 2 to get 7
        let bound = (upto as f64).sqrt() as uint + 1;
        while check <= bound {
            if is_prime[check] {
                let p = 2 * check + 1;
                for zero in iter::range_step(2 * check * (check + 1), upto / 2, p) {
                    is_prime.set(zero, false);
                }
            }

            check += tick;
            tick = 3 - tick;
        }

        Primes { v: is_prime }
    }

    /// The largest number stored.
    pub fn upper_bound(&self) -> uint {
        self.v.len() * 2 + 1
    }

    /// Check if `n` is prime, failing if `n` is larger than the upper
    /// bound of this Primes instance.
    pub fn is_prime(&self, n: uint) -> bool {
        assert!(n <= self.upper_bound());
        if n % 2 == 0 {
            // 2 is the evenest prime.
            n == 2
        } else {
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
    /// Returns None if `n` cannot be factored, specifically if `n` is
    /// zero, or if `n` has primes factors larger than the largest
    /// stored in this sieve.
    pub fn factor(&self, mut n: uint) -> Option<Vec<(uint, u32)>> {
        if n == 0 { return None }

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

        if n == 1 {
            Some(ret)
        } else {
            // large factors! :(
            None
        }
    }
}

/// Returns estimated bounds for π(n), the number of primes less than
/// or equal to `n`.
///
/// That is, if `(a, b) = estimate_prime_pi(n)`, `a ≤ π(n) ≤ b`.
pub fn estimate_prime_pi(n: uint) -> (uint, uint) {
    if n >= 55 {
        let n = n as f64;
        let lg = n.ln();

        ((n / (lg + 2.0)) as uint, (n / (lg - 4.0)) as uint)
    } else {
        static SMALL_PRIME_PI: [uint, .. 55] =
            [0, 0, /*2*/1, /*3*/2, 2, /*5*/3, 3, /*7*/ 4, 4, 4,
             4, /*11*/5, 5, /*13*/ 6, 6, 6, 6, /*17*/ 7, 7, /*19*/8,
             8, 8, 8, /*23*/9, 9, 9, 9, 9, 9, /*29*/10,
             10, /*31*/11, 11, 11, 11, 11, 11, /*37*/12, 12, 12,
             12, /*41*/13, 13, /*43*/14, 14, 14, 14, /*47*/15, 15, 15,
             15, 15, 15, /*53*/16, 16];
        let x = SMALL_PRIME_PI[n];
        (x, x)
    }
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
                let (below_hi, above_hi) = estimate_prime_pi(hi);
                let (below_lo, above_lo) = estimate_prime_pi(lo);

                (below_hi - cmp::min(above_lo, below_hi),
                 Some(above_hi - below_lo + 1))
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

    use std::iter::range_step;
    use super::{Primes, estimate_prime_pi};
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

        let tests: &[(uint, &[(uint, u32)])] = &[
            (1, &[]),
            (2, &[(2u, 1u32)]),
            (3, &[(3, 1)]),
            (4, &[(2, 2)]),
            (5, &[(5, 1)]),
            (6, &[(2, 1), (3, 1)]),
            (7, &[(7, 1)]),
            (8, &[(2, 3)]),
            (9, &[(3, 2)]),
            (10, &[(2, 1), (5, 1)]),
            ];
        for &(n, expected) in tests.iter() {
            assert_eq!(primes.factor(n), Some(expected.to_vec()));
        }
    }

    #[test]
    fn factor_failures() {
        let primes = Primes::sieve(30);

        assert_eq!(primes.factor(0), None);
        assert_eq!(primes.factor(97), None);
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
        let primes = Primes::sieve(1_000_000);

        let mut last = 0;
        for (i, p) in primes.primes().enumerate() {
            for j in range(last, p) {
                let (lo, hi) = estimate_prime_pi(j);
                assert!(lo <= i && i <= hi,
                        "found failing estimate at {}, should satisfy: {} <= {} <= {}",
                        j, lo, i, hi)
            }
            last = p;
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
}
