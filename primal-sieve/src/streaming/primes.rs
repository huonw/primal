use crate::wheel;
use crate::streaming::{self, StreamingSieve};

#[cfg(target_pointer_width = "32")]
const SQRT: usize = 1 << 16;
#[cfg(target_pointer_width = "64")]
const SQRT: usize = 1 << 32;

enum Early {
    Two,
    Three,
    Five,
    Done,
}

/// An iterator over all primes.
///
/// This will yield primes indefinitely (bits in `usize`
/// permitting). If there is an known upper bound, sieving first with
/// `Sieve` and using its `primes_from` method may be more efficient,
/// especially if the bound is small.
///
/// This requires *O(sqrt(p))* memory to yield prime `p`, where `X` is
/// the maximum value of `usize`.
///
/// # Examples
///
/// ```rust
/// let count = primal::Primes::all().take_while(|p| *p < 1_000_000).count();
/// println!("{}", count);
/// ```
pub struct Primes {
    early: Early,
    base: usize,
    ones: primal_bit::IntoOnes,
    streaming: StreamingSieve,
    sieving_primes: Option<Box<Primes>>,
    left_over: Option<usize>,
}

impl Primes {
    /// The sequence `2, 3, 5, 7, 11, ...`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // print the first 20 primes
    /// for p in primal::Primes::all().take(20) {
    ///     println!("{}", p);
    /// }
    /// ```
    pub fn all() -> Primes {
        Primes::sqrt(SQRT)
    }

    fn sqrt(sqrt: usize) -> Primes {
        let (sieving, limit) = if sqrt < streaming::isqrt(streaming::SEG_LEN) {
            (None, sqrt * sqrt)
        } else {
            (Some(Box::new(Primes::sqrt(streaming::isqrt(sqrt)))),
             streaming::SEG_LEN)
        };
        let mut streaming = StreamingSieve::new(limit);

        let ones = {
            let (_, bits) = streaming.next().unwrap();
            bits.clone().into_ones()
        };

        // we manually add the primes
        streaming.small = None;
        // go to the end.
        streaming.limit = !0;

        Primes {
            early: Early::Two,
            base: 0,
            ones,
            streaming,
            sieving_primes: sieving,
            left_over: None,
        }
    }

    #[inline]
    fn from_bit_index(&self, i: usize) -> Option<usize> {
        let w = wheel::from_bit_index(i);
        self.base.checked_add(w)
    }

    fn advance_ones(&mut self) -> bool {
        let low = self.streaming.low;
        let high = low.saturating_add(streaming::SEG_LEN);

        for q in self.left_over.into_iter().chain(self.sieving_primes.as_mut().unwrap()) {
            if q.saturating_mul(q) >= high {
                self.left_over = Some(q);
                break
            }
            if q > streaming::isqrt(streaming::SEG_LEN) {
                self.streaming.add_sieving_prime(q, low);
            }
        }

        match self.streaming.next() {
            Some((_, bits)) => {
                self.base = low;
                self.ones = bits.clone().into_ones();
                true
            },
            None => false,
        }
    }
}

// See also `Iterator for SievePrimes` with nearly identical code.
impl Iterator for Primes {
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
    use crate::Sieve;
    use super::Primes;

    fn check_equality(limit: usize) {
        let sieve = Sieve::new(limit);

        let real = sieve.primes_from(0).take_while(|x| *x < limit);
        let computed = Primes::all().take_while(|x| *x < limit);

        let mut i = 0;
        for (r, c) in real.zip(computed) {
            assert_eq!(c, r);
            i += 1;
        }
        assert_eq!(sieve.prime_pi(limit), i);
    }

    #[test]
    fn equality() {
        check_equality(20_000_000);
    }

    #[test]
    fn equality_huge() {
        let limit = if cfg!(feature = "slow_tests") {
            // This takes a minute or so in debug mode, but it does work!
            ::std::u32::MAX as usize
        } else {
            100_000_000
        };
        check_equality(limit);
    }

    #[test]
    #[should_panic = "123456791"]
    fn fold() {
        // There's no termination until we exceed `usize::MAX`, which
        // will take too long, but we can cut it short by unwinding.
        Primes::all().fold((), |(), p| {
            if p > 123456789 {
                panic!("{}", p);
            }
        })
    }
}
