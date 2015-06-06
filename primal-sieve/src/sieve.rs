use primal_bit::BitVec;
use wheel;
use streaming::StreamingSieve;
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
        let mut stream = StreamingSieve::new(limit);

        let mut seen = Vec::new();
        let mut nbits = 0;
        while let Some((n, bits)) = stream.next() {
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
        let last_bit = self.nbits - 1;
        wheel::from_bit_index(last_bit)
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
    fn upper_bound() {
        let primes = Sieve::new(30);
        assert!(primes.upper_bound() >= 29);
        let primes = Sieve::new(31);
        assert!(primes.upper_bound() >= 31);

        let primes = Sieve::new(30000);
        assert!(primes.upper_bound() >= 29999);
        let primes = Sieve::new(30001);
        assert!(primes.upper_bound() >= 30001);
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
}
