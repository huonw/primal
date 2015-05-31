use primal_bit::BitVec;
use wheel;
use streaming::StreamingSieve;

use std::cmp;

pub struct Sieve {
    #[allow(dead_code)]
    stream: StreamingSieve,
    nbits: usize,
    seen: Vec<BitVec>,
}

impl Sieve {
    pub fn new(limit: usize) -> Sieve {
        let mut stream = StreamingSieve::new(limit);

        let mut seen = Vec::new();
        let mut nbits = 0;
        while let Some((n, bits)) = stream.next() {
            seen.push(bits.clone());
            nbits += cmp::min(bits.len(), wheel::bit_index(limit - n + 1).1);
        }
        Sieve {
            stream: stream,
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
    pub fn upper_bound(&self) -> usize {
        let last_bit = self.nbits - 1;
        wheel::from_bit_index(last_bit)
    }
    pub fn is_prime(&self, n: usize) -> bool {
        match self.index_for(n) {
            (false, _, _) => n == 2 || n == 3 || n == 5 || n == 7,
            (true, base, tweak) => !self.seen[base][tweak],
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
}
