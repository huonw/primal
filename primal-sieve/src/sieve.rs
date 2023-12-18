use primal_bit::BitVec;
use crate::wheel;
use crate::streaming::StreamingSieve;

use core::cmp;
use core::slice;

#[cfg(feature = "no-std")]
use alloc::{vec, vec::Vec};

type SmallVec1<T> = ::smallvec::SmallVec<[T; 1]>;

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
/// let sieve = primal::Sieve::new(10_000_000);
/// assert_eq!(sieve.prime_pi(123456), 11601);
///
/// assert!(sieve.is_prime(6395047));
/// assert!(!sieve.is_prime(6395048));
/// ```
#[derive(Debug)]
pub struct Sieve {
    seg_bits: usize,
    nbits: usize,
    seen: SmallVec1<Item>,
}

#[derive(Debug)]
struct Item {
    count: usize,
    bits: BitVec,
}
impl Item {
    fn new(x: BitVec, so_far: &mut usize) -> Item {
        *so_far += x.count_ones();
        Item {
            count: *so_far,
            bits: x,
        }
    }
}

impl Sieve {
    /// Create a new instance, sieving out all the primes up to
    /// `limit`.
    pub fn new(limit: usize) -> Sieve {
        let mut seen = SmallVec1::new();
        let mut nbits = 0;
        let mut so_far = 0;
        let mut seg_bits = None;
        match wheel::small_for(limit) {
            Some(bits) => {
                nbits = bits.len();
                seen.push(Item::new(bits, &mut 0));
                seg_bits = Some(nbits)
            }
            None => {
                let mut stream = StreamingSieve::new(limit);

                while let Some((n, bits)) = stream.next() {
                    let bits_limit = wheel::bit_index((limit - n).saturating_add(1)).1;
                    seen.push(Item::new(bits.clone(), &mut so_far));
                    nbits += cmp::min(bits.len(), bits_limit);
                    match seg_bits {
                        None => seg_bits = Some(bits.len()),
                        Some(old) => assert_eq!(old, bits.len()),
                    }
                }
            }
        }
        // this is a bit of a lie, but this length only matters when
        // computing indices into `seen`, and everything will be in
        // the first and only one in this case, so we better ensure
        // that all queries get fed into that array (there's been
        // panics from the limit being used as a query for
        // e.g. prime_pi, as split_index would return (1, 0),
        // suggesting that code look at a non-existant element of
        // seen).
        let seg_bits_adjust = if seen.len() == 1 { 1 } else { 0 };

        Sieve {
            seg_bits: seg_bits.unwrap() + seg_bits_adjust,
            nbits,
            seen,
        }
    }
    fn split_index(&self, idx: usize) -> (usize, usize) {
        (idx / self.seg_bits, idx % self.seg_bits)
    }
    fn index_for(&self, n: usize) -> (bool, usize, usize) {
        let (b, idx) = wheel::bit_index(n);
        let (base, tweak) = self.split_index(idx);
        (b, base, tweak)
    }

    fn prime_pi_chunk(&self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            self.seen[n - 1].count
        }
    }
    /// Return the largest number that this sieve knows about.
    ///
    /// It will be at least as large as the number passed to `new`,
    /// but may be slightly larger.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let sieve = primal::Sieve::new(1000);
    ///
    /// assert!(sieve.upper_bound() >= 1000);
    /// ```
    pub fn upper_bound(&self) -> usize {
        wheel::upper_bound(self.nbits)
    }

    /// Determine if `n` is a prime number.
    ///
    /// # Panics
    ///
    /// If `n` is out of range (greater than `self.upper_bound()`),
    /// `is_prime` will panic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let sieve = primal::Sieve::new(1000);
    ///
    /// assert_eq!(sieve.is_prime(0), false);
    /// assert_eq!(sieve.is_prime(1), false);
    /// assert_eq!(sieve.is_prime(2), true);
    /// assert_eq!(sieve.is_prime(3), true);
    /// assert_eq!(sieve.is_prime(4), false);
    /// assert_eq!(sieve.is_prime(5), true);
    ///
    /// assert_eq!(sieve.is_prime(991), true);
    /// assert_eq!(sieve.is_prime(993), false);
    /// assert_eq!(sieve.is_prime(995), false);
    /// assert_eq!(sieve.is_prime(997), true);
    /// assert_eq!(sieve.is_prime(999), false);
    /// ```
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
    /// `prime_pi` will panic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let sieve = primal::Sieve::new(1000);
    ///
    /// assert_eq!(sieve.prime_pi(10), 4);
    /// // the endpoint is included
    /// assert_eq!(sieve.prime_pi(11), 5);
    ///
    /// assert_eq!(sieve.prime_pi(100), 25);
    /// assert_eq!(sieve.prime_pi(1000), 168);
    /// ```
    pub fn prime_pi(&self, n: usize) -> usize {
        assert!(n <= self.upper_bound());
        match n {
            0..=1 => 0,
            2 => 1,
            3..=4 => 2,
            5..=6 => 3,
            7..=10 => 4,
            _ => {
                let (includes, base, tweak) = self.index_for(n);
                let mut count = match wheel::BYTE_MODULO {
                    30 => 3,
                    _ => unimplemented!()
                };

                count += self.prime_pi_chunk(base);
                count += self.seen[base].bits.count_ones_before(tweak + includes as usize);

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
    ///
    /// # Examples
    ///
    /// ```rust
    /// let sieve = primal::Sieve::new(100);
    ///
    /// assert_eq!(sieve.factor(2), Ok(vec![(2, 1)]));
    /// assert_eq!(sieve.factor(4), Ok(vec![(2, 2)]));
    /// assert_eq!(sieve.factor(1 << 31), Ok(vec![(2, 31)]));
    ///
    /// assert_eq!(sieve.factor(18), Ok(vec![(2, 1), (3, 2)]));
    ///
    /// assert_eq!(sieve.factor(25 * 81), Ok(vec![(3, 4), (5, 2)]));
    ///
    /// // "large" prime factors are OK, as long as there's only one
    /// assert_eq!(sieve.factor(2 * 3 * 97 * 97 * 991),
    ///            Ok(vec![(2, 1), (3, 1), (97, 2), (991, 1)]));
    ///
    /// // too many large factors is problematic
    /// assert_eq!(sieve.factor(991 * 991),
    ///            Err((991 * 991, vec![])));
    /// assert_eq!(sieve.factor(2 * 3 * 17 * 17 * 991 * 991),
    ///            Err((991 * 991, vec![(2, 1), (3, 1), (17, 2)])));
    /// ```
    pub fn factor(&self, mut n: usize) -> Result<Vec<(usize,usize)>,
                                                 (usize, Vec<(usize, usize)>)>
    {
        if n == 0 { return Err((0, vec![])) }
        if n == 1 { return Ok(vec![]) }

        let mut ret = Vec::new();

        // Using optimized internal iteration
        self.primes_from(0).for_each_while(|p| {
            if n % p == 0 {
                n /= p;
                let mut count = 1;
                while n % p == 0 {
                    n /= p;
                    count += 1;
                }
                ret.push((p,count));
            }

            p.saturating_mul(p) < n
        });

        if n != 1 {
            let b = self.upper_bound();
            if let Some(bb) = b.checked_mul(b) {
                if bb < n {
                    // large factors :(
                    return Err((n, ret))
                }
            }

            // n is not divisible by anything from 1..=sqrt(n), so
            // must be prime itself! (That is, even though we
            // don't know this prime specifically, we can infer
            // that it must be prime.)
            ret.push((n, 1));
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
    /// `self.prime_pi(self.upper_bound())`).
    ///
    /// # Example
    ///
    /// ```rust
    /// let (_, hi) = primal::estimate_nth_prime(1_000);
    ///
    /// let sieve = primal::Sieve::new(hi as usize);
    ///
    /// assert_eq!(sieve.nth_prime(10), 29);
    /// assert_eq!(sieve.nth_prime(100), 541);
    /// assert_eq!(sieve.nth_prime(1_000), 7919);
    /// ```
    pub fn nth_prime(&self, n: usize) -> usize {
        match n {
            1 => 2,
            2 => 3,
            3 => 5,
            _ => {
                assert!(0 < n && n <= 3 + self.prime_pi_chunk(self.seen.len()));
                // the bit vectors don't store the first three primes,
                // so we're looking for this (one-indexed) bit
                let bit_n = n - 3;

                let chunk_idx = self.seen.binary_search_by(|x| x.count.cmp(&bit_n))
                                         .unwrap_or_else(|x| x);
                let chunk_bits = self.prime_pi_chunk(chunk_idx);
                let bit_idx = self.seen[chunk_idx].bits.find_nth_bit(bit_n - chunk_bits - 1);
                wheel::from_bit_index(chunk_idx * self.seg_bits + bit_idx.unwrap())
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
    /// `primes_from` will panic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let sieve = primal::Sieve::new(1_000);
    ///
    /// // print the three digit primes
    /// for p in sieve.primes_from(100).take_while(|x| *x < 1000) {
    ///     println!("{}", p);
    /// }
    /// ```
    pub fn primes_from(&self, n: usize) -> SievePrimes<'_> {
        assert!(n <= self.upper_bound());
        let early = match n {
            0..=2 => Early::Two,
            3 => Early::Three,
            4..=5 => Early::Five,
            _ => Early::Done
        };
        let (_, base, tweak) = self.index_for(n);
        assert!(self.seen.len() == 1 || self.seg_bits % 8 == 0);
        let base_byte_count = base * self.seg_bits / 8;

        let bits = &self.seen[base].bits;

        let current_base = base_byte_count * wheel::BYTE_MODULO;
        let next_base = current_base + bits.len() * wheel::BYTE_MODULO / 8;

        SievePrimes {
            early,
            base: current_base,
            next_base,
            ones: bits.ones_from(tweak),
            limit: self.upper_bound(),
            bits: self.seen[base + 1..].iter(),
        }
    }
}

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
    next_base: usize,
    limit: usize,
    ones: primal_bit::Ones<'a>,
    bits: slice::Iter<'a, Item>,
}

impl<'a> SievePrimes<'a> {
    #[inline]
    fn from_bit_index(&self, i: usize) -> Option<usize> {
        let w = wheel::from_bit_index(i);
        match self.base.checked_add(w) {
            Some(p) if p <= self.limit => Some(p),
            _ => None,
        }
    }

    fn advance_ones(&mut self) -> bool {
        match self.bits.next() {
            Some(Item { bits, .. }) => {
                self.base = self.next_base;
                self.next_base += bits.len() * wheel::BYTE_MODULO / 8;
                self.ones = bits.ones_from(0);
                true
            },
            None => false,
        }
    }

    // Private method specifically to get internal iteration in `factor`.
    // When `Try` is stable, we could more generally override `try_fold`, but
    // that also requires keeping all state consistent, like `self.early`.
    fn for_each_while<F>(mut self, mut f: F)
    where
        F: FnMut(usize) -> bool,
    {
        if !match self.early {
            Early::Done => true,
            Early::Two => f(2) && f(3) && f(5),
            Early::Three => f(3) && f(5),
            Early::Five => f(5),
        } {
            return;
        }
        loop {
            while let Some(i) = self.ones.next() {
                match self.from_bit_index(i) {
                    Some(p) => if !f(p) { return },
                    None => return,
                }
            }
            if !self.advance_ones() {
                return;
            }
        }
    }
}

// See also `Iterator for Primes` with nearly identical code.
impl<'a> Iterator for SievePrimes<'a> {
    type Item = usize;

    #[inline]
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
        loop {
            if let Some(i) = self.ones.next() {
                return self.from_bit_index(i);
            }
            if !self.advance_ones() {
                return None;
            }
        }
    }

    fn fold<Acc, F>(mut self, mut acc: Acc, mut f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc
    {
        match self.early {
            Early::Done => {}
            Early::Two => {
                acc = f(acc, 2);
                acc = f(acc, 3);
                acc = f(acc, 5);
            }
            Early::Three => {
                acc = f(acc, 3);
                acc = f(acc, 5);
            }
            Early::Five => {
                acc = f(acc, 5);
            }
        }
        loop {
            while let Some(i) = self.ones.next() {
                match self.from_bit_index(i) {
                    Some(p) => acc = f(acc, p),
                    None => return acc,
                }
            }
            if !self.advance_ones() {
                return acc;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use primal_slowsieve::Primes;
    use super::Sieve;

    #[test]
    fn small() {
        let larger = Sieve::new(100_000);
        for limit in 2..1_000 {
            let sieve = Sieve::new(limit);
            assert!(sieve.upper_bound() >= limit);
            let primes = sieve.prime_pi(limit);
            assert_eq!(primes, larger.prime_pi(limit));
            let nth = sieve.nth_prime(primes);
            assert!(nth <= limit);
            assert_eq!(nth, larger.nth_prime(primes));
        }
    }

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
                   primes.prime_pi(upto));
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
        assert_eq!(i, primes.prime_pi(limit));
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

        let range = if cfg!(feature = "slow_tests") {
            1..200
        } else {
            100..120
        };
        for i in range {
            let i = i * 10_000;
            let primes = Sieve::new(i);
            assert!(primes.upper_bound() >= i);
        }
    }

    #[test]
    fn prime_pi() {
        let (limit, mult) = if cfg!(feature = "slow_tests") {
            (2_000_000, 19_998)
        } else {
            (200_000, 1_998)
        };
        let primes = Sieve::new(limit);
        let real = Primes::sieve(limit);

        for i in (0..20).chain((0..100).map(|n| n * mult + 1)) {
            let val = primes.prime_pi(i);
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
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn factor_overflow() {
        // if bound^2 overflows usize, we can factor any usize,
        // but must take care to not hit overflow assertions.

        // set up a limit that would overflow if naively squared, and a
        // prime greater than that limit.  (these are more than double)
        #[cfg(target_pointer_width = "32")]
        const LIMIT_PRIME: (usize, usize) = (0x10000, 0x2001d);
        #[cfg(target_pointer_width = "64")]
        const LIMIT_PRIME: (usize, usize) = (0x100000000, 0x200000011);

        let (limit, prime) = LIMIT_PRIME;
        let primes = Sieve::new(limit);
        assert!(prime > primes.upper_bound());
        assert_eq!(primes.factor(prime), Ok(vec![(prime, 1)]));
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
        let total = primes.prime_pi(primes.upper_bound());
        assert!(primes.nth_prime(total) <= primes.upper_bound());
    }

    #[test]
    fn sum_primes() {
        let primes = Sieve::new(2_000_000);

        let mut manual_sum = 0u64;
        for p in primes.primes_from(0) {
            manual_sum += p as u64;
        }
        dbg!(manual_sum);

        let folded_sum = primes.primes_from(0).fold(0u64, |acc, p| acc + p as u64);
        let trait_sum = primes.primes_from(0).map(|p| p as u64).sum::<u64>();
        assert_eq!(manual_sum, folded_sum);
        assert_eq!(manual_sum, trait_sum);
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn u32_primes() {
        const COUNT: usize = 203_280_221; // number of 32-bit primes
        const LAST: usize = 4_294_967_291; // last 32-bit prime
        const SUM: u64 = 425_649_736_193_687_430; // sum of 32-bit primes

        let sieve = Sieve::new(core::u32::MAX as usize);
        assert!(sieve.upper_bound() >= LAST);
        assert_eq!(sieve.primes_from(LAST - 100).last(), Some(LAST));

        let mut count = 0;
        let mut sum = 0;
        for p in sieve.primes_from(0) {
            count += 1;
            sum += p as u64;
        }
        assert_eq!(count, COUNT);
        assert_eq!(sum, SUM);
    }

    #[test]
    fn prime_pi_sieve_limit() {
        // previously, these numbers would result in an index
        // out-of-bounds when used as the limit and the number fed to
        // prime_pi.
        for limit in 19998..20004 {
            let sieve = Sieve::new(limit);
            sieve.prime_pi(limit);
        }
    }
}
