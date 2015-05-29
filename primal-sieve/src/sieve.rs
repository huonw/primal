use primal_estimate;
use primal_bit::{self, BitVec};
use std::{iter, cmp};

use streaming::StreamingSieve;

pub struct Sieve {
    #[allow(dead_code)]
    stream: StreamingSieve,
    seen: BitVec,
}

/// Iterator over the primes stored in a sieve.
#[derive(Clone)]
pub struct PrimeIterator<'a> {
    two: bool,
    iter: iter::Enumerate<primal_bit::Iter<'a>>,
}

impl Sieve {
    pub fn new(limit: usize) -> Sieve {
        let mut stream = StreamingSieve::new(limit);

        let mut seen = BitVec::with_capacity(limit / 2);
        while let Some((n, bits)) = stream.next() {
            seen.push_all(&bits, (limit - n + 1) / 2);
        }
        Sieve {
            stream: stream,
            seen: seen,
        }
    }
    pub fn upper_bound(&self) -> usize {
        (self.seen.len() - 1) * 2 + 1
    }
    pub fn is_prime(&self, n: usize) -> bool {
        if n % 2 == 0 {
            n == 2
        } else {
            assert!(n <= self.upper_bound());
            !self.seen[n / 2]
        }
    }

    /// Iterator over the primes stored in this map.
    pub fn primes<'a>(&'a self) -> PrimeIterator<'a> {
        PrimeIterator {
            two: true,
            iter: self.seen.iter().enumerate()
        }
    }
}

impl<'a> Iterator for PrimeIterator<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.two {
            self.two = false;
            Some(2)
        } else {
            for (i, is_not_prime) in &mut self.iter {
                if !is_not_prime {
                    return Some(2 * i + 1)
                }
            }
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut iter = self.clone();
        // TODO: this doesn't run in constant time, is it super-bad?
        match (iter.next(), iter.next_back()) {
            (Some(lo), Some(hi)) => {
                let (below_hi, above_hi) = primal_estimate::prime_pi(hi as u64);
                let (below_lo, above_lo) = primal_estimate::prime_pi(lo as u64);

                ((below_hi - cmp::min(above_lo, below_hi)) as usize,
                 Some((above_hi - below_lo + 1) as usize))
            }
            (Some(_), None) => (1, Some(1)),
            (None, _) => (0, Some(0))
        }
    }
}

impl<'a> DoubleEndedIterator for PrimeIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<usize> {
        loop {
            match self.iter.next_back() {
                Some((i, false)) => return Some(2 * i + 1),
                Some((_, true)) => {/* continue */}
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
    use primal_smallsieve::Primes;
    use super::Sieve;

    #[test]
    fn is_prime() {
        let limit = 1_000_000;
        let real = Primes::sieve(limit);
        let primes = Sieve::new(limit);

        for i in 0..limit {
            assert_eq!(primes.is_prime(i),
                       real.is_prime(i))
        }
    }

    #[test]
    fn upper_bound() {
        let primes = Sieve::new(30);
        assert_eq!(primes.upper_bound(), 29);
        let primes = Sieve::new(31);
        assert_eq!(primes.upper_bound(), 31);

        let primes = Sieve::new(30000);
        assert_eq!(primes.upper_bound(), 29999);
        let primes = Sieve::new(30001);
        assert_eq!(primes.upper_bound(), 30001);
    }

    #[test]
    fn primes_iterator() {
        let primes = Sieve::new(50);
        let mut expected = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

        assert_eq!(primes.primes().collect::<Vec<usize>>(), expected);

        expected.reverse();
        assert_eq!(primes.primes().rev().collect::<Vec<usize>>(), expected);
    }

    #[test]
    fn size_hint() {
        let mut i = 0;
        while i < 1000 {
            let sieve = Sieve::new(i);

            let mut primes = sieve.primes();

            // check the size hint at each and every iteration
            loop {
                let (lo, hi) = primes.size_hint();

                let copy = primes.clone();
                let len = copy.count();

                let next = primes.next();

                assert!(lo <= len && len <= hi.unwrap(),
                        "found failing size_hint for {:?} to {}, should satisfy: {} <= {} <= {:?}",
                        next, i, lo, len, hi);

                if next.is_none() {
                    break
                }
            }

            i += 100;
        }
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    use super::Sieve;
    use test::Bencher;

    #[bench]
    fn sieve_small(b: &mut Bencher) {
        b.iter(|| Sieve::new(100))
    }
    #[bench]
    fn sieve_medium(b: &mut Bencher) {
        b.iter(|| Sieve::new(10_000))
    }
    #[bench]
    fn sieve_large(b: &mut Bencher) {
        b.iter(|| Sieve::new(100_000))
    }
    #[bench]
    fn sieve_huge(b: &mut Bencher) {
        b.iter(|| Sieve::new(10_000_000))
    }

    fn bench_iterate(b: &mut Bencher, upto: usize) {
        let sieve = Sieve::new(upto);

        b.iter(|| {
            sieve.primes().count()
        })
    }

    #[bench]
    fn iterate_small(b: &mut Bencher) { bench_iterate(b, 100) }
    #[bench]
    fn iterate_large(b: &mut Bencher) { bench_iterate(b, 100_000) }
}
