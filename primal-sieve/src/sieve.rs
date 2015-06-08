use primal_bit::BitVec;
use wheel;
use streaming;
use hamming;

use std::cmp::{self, Ordering};

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
    seen: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    count: usize,
    bits: BitVec,
}
impl Item {
    fn new(x: &BitVec, so_far: &mut usize) -> Item {
        *so_far += hamming::weight(x.as_bytes()) as usize;
        Item {
            count: *so_far,
            bits: x.clone()
        }
    }
}

impl Sieve {
    /// Create a new instance, sieving out all the primes up to
    /// `limit`.
    pub fn new(limit: usize) -> Sieve {
        let mut stream = streaming::new(limit);

        let mut seen = Vec::new();
        let mut nbits = 0;
        let mut so_far = 0;
        while let Some((n, bits)) = streaming::next(&mut stream) {
            seen.push(Item::new(bits, &mut so_far));
            nbits += cmp::min(bits.len(), wheel::bit_index(limit - n + 1).1);
        }
        Sieve {
            nbits: nbits,
            seen: seen,
        }
    }
    fn split_index(&self, idx: usize) -> (usize, usize) {
        let len = self.seen[0].bits.len();
        (idx / len,idx % len)
    }
    fn index_for(&self, n: usize) -> (bool, usize, usize) {
        let (b, idx) = wheel::bit_index(n);
        let (base, tweak) = self.split_index(idx);
        (b, base, tweak)
    }

    fn count_upto_chunk(&self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            self.seen[n - 1].count
        }
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
            (true, base, tweak) => self.seen[base].bits[tweak],
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

                count += self.count_upto_chunk(base);
                let (tweak_byte, tweak_bit) = (tweak / 8, tweak % 8);

                let bytes = self.seen[base].bits.as_bytes();
                count += hamming::weight(&bytes[..tweak_byte]) as usize;
                let byte = bytes[tweak_byte];
                for i in 0..tweak_bit + includes as usize {
                    count += (byte & (1 << i) != 0) as usize
                }
                count
            }
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
    pub fn factor(&self, mut n: usize) -> Result<Vec<(usize,usize)>,
                                                 (usize, Vec<(usize, usize)>)>
    {
        if n == 0 { return Err((0, vec![])) }

        let mut ret = Vec::new();

        for p in self.primes_from(0) {
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

    /// Compute *p<sub>n</sub>*, the `n` prime number, 1-indexed
    /// (i.e. *p<sub>1</sub>* = 2, *p<sub>2</sub>* = 3).
    ///
    /// # Panics
    ///
    /// `n` must be larger than 0 and less than the total number of
    /// primes in this sieve (that is,
    /// `self.count_upto(self.upper_bound())`).
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate primal;
    /// let (_, hi) = primal::estimate_nth_prime(1_000);
    ///
    /// let sieve = primal::Sieve::new(hi as usize);
    /// assert_eq!(sieve.nth_prime(1_000), 7919);
    /// ```
    pub fn nth_prime(&self, n: usize) -> usize {
        assert!(0 < n && n <= self.count_upto_chunk(self.seen.len()));
        match n {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => {
                // the bit vectors don't store the first three primes,
                // so we're looking for this (one-indexed) bit
                let bit_n = n - 3;

                let chunk_idx = self.seen.binary_search_by(|x| x.count.cmp(&bit_n))
                                         .unwrap_or_else(|x| x);
                let chunk_bits = self.count_upto_chunk(chunk_idx);
                let mut bit = bit_n - chunk_bits;
                let all_bytes = self.seen[chunk_idx].bits.as_bytes();
                let mut bytes = all_bytes;

                while bytes.len() > 240 {
                    let ix = bytes.len() / 2;
                    let (first, second) = bytes.split_at(ix);

                    let count = hamming::weight(first) as usize;
                    match count.cmp(&bit) {
                        Ordering::Equal | Ordering::Greater => {
                            bytes = first;
                        }
                        Ordering::Less => {
                            bit -= count;
                            bytes = second;
                        }
                    }
                }

                let mut byte_idx = bytes.as_ptr() as usize - all_bytes.as_ptr() as usize;

                let mut b = 0;
                for &b_ in bytes {
                    b = b_;
                    let count = b_.count_ones() as usize;
                    if count >= bit {
                        break
                    }

                    byte_idx += 1;
                    bit -= count
                }
                assert!(b != 0);
                // clear the bottom bit-1 set bits
                for _ in 1..bit {
                    b = b & (b - 1);
                }
                assert!(b != 0);
                let bit_idx = chunk_idx * self.seen[0].bits.len()
                    + byte_idx * 8
                    + b.trailing_zeros() as usize;
                wheel::from_bit_index(bit_idx)
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
    pub fn primes_from<'a>(&'a self, n: usize) -> SievePrimes<'a> {
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
        assert!(self.seen.len() == 1 || self.seen[0].bits.len() % 64 == 0);
        let base_u64_count = base * self.seen[0].bits.len() / 64 + tweak_u64;

        let mut elems = self.seen[base].bits.as_u64s()[tweak_u64..].iter();
        let current = elems.next().unwrap() & tweak_mask;

        SievePrimes {
            early: early,
            base: base_u64_count * ITER_BASE_STEP,
            current: current,
            elems: elems,
            limit: self.upper_bound(),
            bits: self.seen[base + 1..].iter(),
        }
    }
}

use std::slice;
const ITER_BASE_STEP: usize = 8 * wheel::BYTE_MODULO;

#[derive(Clone)]
enum Early {
    Two,
    Three,
    Five,
    Done,
}

/// An iterator over the primes stored in a `Sieve` instance.
#[derive(Clone)]
pub struct SievePrimes<'a> {
    early: Early,
    base: usize,
    current: u64,
    limit: usize,
    elems: slice::Iter<'a, u64>,
    bits: slice::Iter<'a, Item>,
}

impl<'a> Iterator for SievePrimes<'a> {
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
                if next != 0 {
                    c = next;
                    break 'find_c
                }
            }
            match self.bits.next() {
                Some(bits) => self.elems = bits.bits.as_u64s().iter(),
                None => return None,
            }
        }

        let lsb = c.trailing_zeros();
        self.current = c & (c - 1);
        let p = self.base + wheel::TRUE_AT_BIT_64[lsb as usize];
        if p <= self.limit {
            Some(p)
        } else {
            self.current = 0;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use primal_slowsieve::Primes;
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
    fn primes_from_no_overrun() {
        let real = Sieve::new(1000);

        for i in 0..100 {
            let i = i * 38 / 39 + 1;
            let sieve = Sieve::new(i);

            for p in sieve.primes_from(0) {
                assert!(real.is_prime(p));
            }
        }
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

    #[test]
    fn factor() {
        let primes = Sieve::new(1000);

        let tests: &[(usize, &[(usize, usize)])] = &[
            (1, &[]),
            (2, &[(2_usize, 1)]),
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
        let short = Sieve::new(30);
        let long = Sieve::new(10000);

        let short_lim = short.upper_bound() * short.upper_bound() + 1;

        // every number less than bound^2 can be factored (since they
        // always have a factor <= bound).
        for n in 0..short_lim {
            assert_eq!(short.factor(n), long.factor(n))
        }
        // larger numbers can only sometimes be factored
        'next_n: for n in short_lim..10000 {
            let possible = short.factor(n);
            let real = long.factor(n).ok().unwrap();

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
                let (low, hi) = real.split_at(last_short_prime);
                let leftover = hi.iter().fold(1, |x, &(p, i)| x * p.pow(i as u32));

                assert_eq!(possible, Err((leftover, low.to_vec())));
                continue 'next_n;
            }

            // if we're here, we know that everything should match
            assert_eq!(possible, Ok(real))
        }
    }

    #[test]
    fn factor_failures() {
        let primes = Sieve::new(30);

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
    fn nth_prime() {
        let primes = Sieve::new(2_000_000);

        for (i, p) in primes.primes_from(0).enumerate() {
            let n = i + 1;
            if n < 2000 || n % 1000 == 0 {
                assert_eq!(primes.nth_prime(n), p);
            }
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
