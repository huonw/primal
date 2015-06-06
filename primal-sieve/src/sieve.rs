use primal_bit::BitVec;
use wheel;
use streaming;
use hamming;

use std::cmp;

/// A heavily optimised prime sieve.
///
/// This stores information about primes up to some specified limit,
/// allowing efficient queries of information about them. This caches
/// the successive outputs of `StreamingSieve` and has very similar
/// performance, modulo the differences in memory use: to cache the
/// information `Sieve` uses approximately `limit / 30 +
/// O(sqrt(limit))` bytes of memory. Consider directly using
/// `StreamingSieve` if repeated queries are unnecessary, since that
/// uses only `O(sqrt(limit))` bytes.
///
/// # Examples
///
/// ```rust
/// # extern crate primal;
/// let sieve = primal::Sieve::new(10_000_000);
/// assert_eq!(sieve.count_upto(123456), 11601);
///
/// assert!(sieve.is_prime(6395047));
/// assert!(!sieve.is_prime(6395048));
/// ```
#[derive(Debug)]
pub struct Sieve {
    nbits: usize,
    seen: Vec<BitVec>,
}

impl Sieve {
    /// Create a new instance, sieving out all the primes up to
    /// `limit`.
    pub fn new(limit: usize) -> Sieve {
        let mut stream = streaming::new(limit);

        let mut seen = Vec::new();
        let mut nbits = 0;
        while let Some((n, bits)) = streaming::next(&mut stream) {
            seen.push(bits.clone());
            nbits += cmp::min(bits.len(), wheel::bit_index(limit - n + 1).1);
        }
        Sieve {
            nbits: nbits,
            seen: seen,
        }
    }
    fn split_index(&self, idx: usize) -> (usize, usize) {
        let len = self.seen[0].len();
        (idx / len,idx % len)
    }
    fn index_for(&self, n: usize) -> (bool, usize, usize) {
        let (b, idx) = wheel::bit_index(n);
        let (base, tweak) = self.split_index(idx);
        (b, base, tweak)
    }
    /// Return the largest number that this sieve knows about.
    pub fn upper_bound(&self) -> usize {
        let last_bit = self.nbits;
        wheel::from_bit_index(last_bit) - 1
    }
    /// Determine if `n` is a prime number.
    ///
    /// # Panics
    ///
    /// If `n` is out of range (greater than `self.upper_bound()`),
    /// `count_upto` will panic.
    pub fn is_prime(&self, n: usize) -> bool {
        match self.index_for(n) {
            (false, _, _) => n == 2 || n == 3 || n == 5 || n == 7,
            (true, base, tweak) => !self.seen[base][tweak],
        }
    }

    /// Count the number of primes upto and including `n`.
    ///
    /// # Panics
    ///
    /// If `n` is out of range (greater than `self.upper_bound()`),
    /// `count_upto` will panic.
    pub fn count_upto(&self, n: usize) -> usize {
        assert!(n <= self.upper_bound());
        match n {
            0...1 => 0,
            2 => 1,
            3...4 => 2,
            5...6 => 3,
            7...10 => 4,
            _ => {
                let (includes, base, tweak) = self.index_for(n);
                let mut count = match wheel::BYTE_MODULO {
                    30 => 3,
                    _ => unimplemented!()
                };

                for v in &self.seen[..base] {
                    let bytes = v.as_bytes();
                    count += 8 * bytes.len() - hamming::weight(bytes) as usize;
                }
                let (tweak_byte, tweak_bit) = (tweak / 8, tweak % 8);

                let bytes = self.seen[base].as_bytes();
                count += 8 * tweak_byte - hamming::weight(&bytes[..tweak_byte]) as usize;
                let byte = bytes[tweak_byte];
                for i in 0..tweak_bit + includes as usize {
                    count += (byte & (1 << i) == 0) as usize
                }
                count
            }
        }
    }

    /// Return an iterator over the primes from `n` (inclusive) to the
    /// end of this sieve.
    ///
    /// NB. it is not guaranteed that the end is the same as the limit
    /// passed to `new`: it may be larger. Consider using `take_while`
    /// if the limit must be exact.
    ///
    /// # Panics
    ///
    /// If `n` is out of range (greater than `self.upper_bound()`),
    /// `count_upto` will panic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate primal;
    /// let sieve = primal::Sieve::new(1_000);
    /// for p in sieve.primes_from(100).take_while(|x| *x <= 1000) {
    ///     println!("{}", p);
    /// }
    /// ```
    pub fn primes_from<'a>(&'a self, n: usize) -> PrimesFrom<'a> {
        assert!(n <= self.upper_bound());
        let early = match n {
            0...2 => Early::Two,
            3 => Early::Three,
            4...5 => Early::Five,
            _ => Early::Done
        };
        let (_, base, tweak) = self.index_for(n);
        let (tweak_u64, tweak_bit) = (tweak / 64, tweak % 64);
        let tweak_mask = (!0) << tweak_bit;
        assert!(self.seen.len() == 1 || self.seen[0].len() % 64 == 0);
        let base_u64_count = base * self.seen[0].len() / 64 + tweak_u64;

        let mut elems = self.seen[base].as_u64s()[tweak_u64..].iter();
        let current = !elems.next().unwrap() & tweak_mask;

        PrimesFrom {
            early: early,
            base: base_u64_count * ITER_BASE_STEP,
            current: current,
            elems: elems,
            bits: self.seen[base + 1..].iter(),
        }
    }
}

use std::slice;
const ITER_BASE_STEP: usize = 8 * wheel::BYTE_MODULO;

enum Early {
    Two,
    Three,
    Five,
    Done,
}

pub struct PrimesFrom<'a> {
    early: Early,
    base: usize,
    current: u64,
    elems: slice::Iter<'a, u64>,
    bits: slice::Iter<'a, BitVec>,
}

impl<'a> Iterator for PrimesFrom<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.early {
            Early::Done => {}
            Early::Two => {
                self.early = Early::Three;
                return Some(2)
            }
            Early::Three => {
                self.early = Early::Five;
                return Some(3)
            }
            Early::Five => {
                self.early = Early::Done;
                return Some(5)
            }
        }
        let mut c = self.current;
        'find_c: while c == 0 {
            for &next in &mut self.elems {
                self.base += ITER_BASE_STEP;
                let next = !next;
                if next != 0 {
                    c = next;
                    break 'find_c
                }
            }
            match self.bits.next() {
                Some(bits) => self.elems = bits.as_u64s().iter(),
                None => return None,
            }
        }

        let lsb = c.trailing_zeros();
        self.current = c & (c - 1);
        Some(self.base + wheel::TRUE_AT_BIT_64[lsb as usize])
    }
}

#[cfg(test)]
mod tests {
    use primal_smallsieve::Primes;
    use super::Sieve;

    #[test]
    fn is_prime() {
        let limit = 2_000_000;
        let real = Primes::sieve(limit);
        let primes = Sieve::new(limit);

        for i in 0..limit {
            assert!(primes.is_prime(i) == real.is_prime(i),
                    "failed for {} (real = {})", i, real.is_prime(i));
        }
    }

    #[test]
    fn primes_from_smoke() {
        let limit = 100;
        let primes = Sieve::new(limit);
        let real = &[2, 3, 5, 7, 11,
                     13, 17, 19, 23, 29,
                     31, 37, 41, 43, 47,
                     53, 59, 61, 67, 71,
                     73, 79, 83, 89, 97];
        for i in 0..limit {
            let idx = real.iter().position(|x| *x >= i).unwrap_or(real.len());
            assert_eq!(primes.primes_from(i).take_while(|x| *x <= limit).collect::<Vec<_>>(),
                       &real[idx..]);
        }
    }
    #[test]
    fn primes_from_count() {
        let limit = 2_100_000;
        let primes = Sieve::new(limit);

        let upto = 2_000_000;
        assert_eq!(primes.primes_from(0).take_while(|x| *x <= upto).count(),
                   primes.count_upto(upto));
    }
    #[test]
    fn primes_from_equality() {
        let limit = 2_000_000;
        let primes = Sieve::new(limit);
        let real = Primes::sieve(limit);

        let real = real.primes().take_while(|x| *x <= limit);
        let computed = primes.primes_from(0).take_while(|x| *x <= limit);
        let mut i = 0;
        for (r, p) in real.zip(computed) {
            assert_eq!(r, p);
            i += 1;
        }
        assert_eq!(i, primes.count_upto(limit));
    }
    #[test]
    fn upper_bound() {
        for i in 1..1000 {
            let primes = Sieve::new(i);
            assert!(primes.upper_bound() >= i);
        }

        for i in 1..200 {
            let i = i * 10000;
            let primes = Sieve::new(i);
            assert!(primes.upper_bound() >= i);
        }
    }

    #[test]
    fn count_upto() {
        let limit = 2_000_000;
        let primes = Sieve::new(limit);
        let real = Primes::sieve(limit);

        for i in (0..20).chain((0..100).map(|n| n * 19998 + 1)) {
            let val = primes.count_upto(i);
            let true_ = real.primes().take_while(|p| *p <= i).count();
            assert!(val == true_, "failed for {}, true {}, computed {}",
                    i, true_, val)
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

    fn count_upto(b: &mut Bencher, n: usize) {
        let s = Sieve::new(n + 1);

        b.iter(|| s.count_upto(n));
    }

    #[bench]
    fn count_upto_small(b: &mut Bencher) { count_upto(b, 100) }
    #[bench]
    fn count_upto_medium(b: &mut Bencher) { count_upto(b, 10_000) }
    #[bench]
    fn count_upto_large(b: &mut Bencher) { count_upto(b, 100_000) }
    #[bench]
    fn count_upto_huge(b: &mut Bencher) { count_upto(b, 10_000_000) }

    fn bench_iterate(b: &mut Bencher, upto: usize) {
        let sieve = Sieve::new(upto);

        b.iter(|| {
            sieve.primes_from(0).count()
        })
    }

    #[bench]
    fn iterate_small(b: &mut Bencher) { bench_iterate(b, 100) }
    #[bench]
    fn iterate_large(b: &mut Bencher) { bench_iterate(b, 100_000) }
    #[bench]
    fn iterate_huge(b: &mut Bencher) { bench_iterate(b, 10_000_000) }
}
