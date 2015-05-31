use primal_bit::BitVec;

use streaming::StreamingSieve;

pub struct Sieve {
    #[allow(dead_code)]
    stream: StreamingSieve,
    seen: BitVec,
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
