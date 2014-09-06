//! Slow and basic handling of primes.

use std::{iter, cmp};
use std::collections::{Bitv, bitv};

#[allow(dead_code)]
mod tables;

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
        // having this out-of-line like this is faster (130 us/iter
        // vs. 111 us/iter on sieve_large), and using a manual while
        // rather than a `range_step` is a similar speedup.
        #[inline(never)]
        fn filter(is_prime: &mut Bitv, upto: uint, check: uint, p: uint) {
            let mut zero = 2 * check * (check + 1);
            while zero < upto / 2 {
                is_prime.set(zero, false);
                zero += p;
            }
        }

        // bad stuff happens for very small bounds.
        let upto = cmp::max(10, upto);

        let mut is_prime = Bitv::with_capacity((upto + 1) / 2, true);
        // 1 isn't prime
        is_prime.set(0, false);

        // multiples of 3 aren't prime (3 is handled separately, so
        // the ticking works properly)
        filter(&mut is_prime, upto, 1, 3);

        let bound = (upto as f64).sqrt() as uint + 1;
        // skip 2.
        let mut check = 2;
        let mut tick = if check % 3 == 1 {2} else {1};

        while check <= bound {
            if is_prime[check] {
                filter(&mut is_prime, upto, check, 2 * check + 1)
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

        let lo = lg + lglg - 1.0 + if n > 13196 {
            // [2] Theorem 1.6
            (lglg - 2.25) / lg
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
    use super::{Primes, estimate_prime_pi, estimate_nth_prime};
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
