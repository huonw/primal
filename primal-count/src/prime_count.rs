extern crate primal_sieve;
use std::iter::FromIterator;

use super::util;
use std::collections::HashMap;
use std::cmp;

const MEISSEL_LOOKUP_SIZE: usize = 8;  // Number of primes we do the reduce trick for
const SMALL_PRIME_PRODUCTS: [usize; MEISSEL_LOOKUP_SIZE + 1] = [1, 2, 6, 30, 210, 2310, 30030, 510510, 9699690];
const SMALL_TOTIENT_VALUES: [usize; MEISSEL_LOOKUP_SIZE + 1] = [1, 1, 2,  8,  48,  480,  5760,  92160, 1658880];

/// Generate a vec of primes from 2 up to and including limit
/// Leverages the fast sieve in primal to do so
fn generate_primes(limit: usize) -> Vec<usize> {
    let sieve = primal_sieve::Sieve::new(limit);
    let sieve_iter = sieve.primes_from(2).take_while(|x| *x <= limit);
    // Note that we put the primes into a vec here because later we want to have both
    //  1) Very quick access to the nth prime
    //  2) Quick counting of number of primes below a value, achieved with a binary search
    // Experiments replacing 1) or 2) with the methods in sieve seem to significantly
    //   slow things down for larger numbers
    return Vec::from_iter(sieve_iter);
}

/// Memoized combinatorial prime counting function
/// Basic idea here: https://en.wikipedia.org/wiki/Meissel%E2%80%93Lehmer_algorithm
/// The "Meissel Function" here is phi on that Wikipedia page
pub struct PrimeCounter {
    limit: usize,
    primes: Vec<usize>,
    pi_cache: HashMap<usize, usize>,
    meissel_cache: HashMap<(usize, usize), usize>
}

impl PrimeCounter {
    /// Create a new PrimeCounter instance, which generates all the primes up to sqrt(limit)
    pub fn new(limit: usize) -> PrimeCounter {
        let mut pi_cache = HashMap::new();
        let primes = generate_primes(util::int_cubic_root(limit * limit));

        // Insert primes <= 10 - this is mainly to deal with underflow issues later
        for n in 0..=10 {
            let nprimes = match n {
                2         => 1,
                3..=4     => 2,
                5..=6     => 3,
                7..=10    => 4,
                0..=1 | _ => 0,  // N.B. _ never hit
            };
            pi_cache.insert(n, nprimes);
        }
        // let meissel_cache = meissel::generate_meissel_lookup(6);
        let meissel_cache = HashMap::new();
        
        PrimeCounter {limit, primes, pi_cache, meissel_cache}
    }

    /// Updates the limit - to be used when you want to make the prime cache larger
    /// May be slow for large values of limit - it's recommended that you don't call
    /// this and instead ensure that your first call to construct the PrimeCounter
    /// object is large enough.
    pub fn update_limit(&mut self, limit: usize) {
        self.limit = limit;
        self.primes = generate_primes(util::int_square_root(limit));
    }
    
    /// The number of primes that are at least `bound`
    /// 
    /// # Panics
    ///
    /// If the limit in the constructor is smaller than the input of prime_pi
    /// (Unless it's later been updated with update_limit)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate primal;
    /// let mut pc = primal::PrimeCounter::new(10_000);
    /// assert_eq!(pc.prime_pi(100), 25);
    /// assert_eq!(pc.prime_pi(8166), 1024);
    /// ```
    pub fn prime_pi(&mut self, bound: usize) -> usize {
        self.primes_less_than(bound)
    }

    /// The number of numbers less than `m` that are coprime to the first `n` prime numbers
    ///
    /// # Panics
    ///
    /// If the `n`th prime is larger than `limit`
    /// 
    /// # Examples
    ///
    /// ```rust
    /// # extern crate primal;
    /// let mut pc = primal::PrimeCounter::new(10_000);
    /// assert_eq!(pc.meissel_fn(100, 10), 16);
    /// assert_eq!(pc.meissel_fn(1234, 5), 256);
    /// ```
    pub fn meissel_fn(&mut self, m: usize, n: usize) -> usize {
        self.meissel_fn_large(m, n)
    }

    /// The number of numbers less than m that are coprime to the first n prime numbers
    /// Get recurrence m_fn(m, n) = m_fn(m, n - 1) - m_fn(m/p_n, n-1) by thinking about p_n, the nth prime
    fn meissel_fn_small(&mut self, m: usize, n: usize) -> usize {
        if n == 0 || m < 2 {
            return m;
        }
        if self.primes[n-1] >= m {
            return 1;
        }
        match self.meissel_cache.get(&(m, n)).map(|entry| entry.clone()){
            Some(result) => result,
            None => {
                // For small values of n, we can decrease the size of m by noting that
                // the meissel function is almost periodic with period p_1 * .. * p_n
                let mut value = 0;
                let mut m_shrunk = m;
                if n <= MEISSEL_LOOKUP_SIZE {
                    value = (m / SMALL_PRIME_PRODUCTS[n]) * SMALL_TOTIENT_VALUES[n];
                    m_shrunk = m_shrunk % SMALL_PRIME_PRODUCTS[n];
                }

                // After shrinkage, just apply the recursion
                value += self.meissel_fn_small(m_shrunk, n-1) - self.meissel_fn_small(m_shrunk / self.primes[n-1], n-1);
                self.meissel_cache.insert((m, n), value);
                return value;
            }
        }
    }

    // Here we make use of the fact that m_fn(m, n) can be evaluated with fewer calls by 
    // taking advantage of n being large
    fn meissel_fn_large(&mut self, m: usize, n: usize) -> usize {
        if n <= MEISSEL_LOOKUP_SIZE { 
            return self.meissel_fn_small(m, n);
        }
        if self.primes[n-1] >= m {
            return 1;
        }
        match self.meissel_cache.get(&(m, n)).map(|entry| entry.clone()){
            Some(result) => result,
            None => {
                let mut result = self.meissel_fn_small(m, MEISSEL_LOOKUP_SIZE);
                let sqrt_m = util::int_square_root(m);
                let bound = self.primes_less_than(sqrt_m);
                let largest_prime_index = cmp::min(cmp::max(bound, MEISSEL_LOOKUP_SIZE), n);
                result = result + largest_prime_index - n;

                for idx in MEISSEL_LOOKUP_SIZE..largest_prime_index {
                    result -= self.meissel_fn_large(m / self.primes[idx], idx);
                }
                self.meissel_cache.insert((m, n), result);
                return result;
            }
        }
    }

    /// Find the number of primes less than bound using the Meissel-Lehmer method
    /// Leverages caching to speed up the recursive calls
    fn primes_less_than(&mut self, bound: usize) -> usize {
        // First check if it's in the cache already
        match self.pi_cache.get(&bound).map(|entry| entry.clone()){
            Some(value) => value,
            None => {
                // The meat of the function
                if bound < 2 {
                    return 0;
                } else if bound <= self.primes[self.primes.len()-1] {
                    let result = match self.primes.binary_search(&bound)
                    {
                        Ok(idx) => idx+1,
                        Err(idx) => idx,
                    };
                    self.pi_cache.insert(bound, result);
                    return result;
                }
                
                let sqrt_bound = util::int_square_root(bound);
                let quartic_bound = util::int_quartic_root(bound);

                let nprimes_below_4thr = self.primes_less_than(quartic_bound);
                let nprimes_below_3rdr = self.primes_less_than(util::int_cubic_root(bound));
                let nprimes_below_2ndr = self.primes_less_than(sqrt_bound);

                // Issues with underflow here if nprimes_below_2ndr + nprimes_below_4thr < 2
                // Dealt with by populating the offending (small) values in the cache at the top level
                let mut result = ((nprimes_below_2ndr + nprimes_below_4thr - 2) * (nprimes_below_2ndr - nprimes_below_4thr + 1)) / 2;
                result += self.meissel_fn_large(bound, nprimes_below_4thr);

                for i in nprimes_below_4thr..nprimes_below_2ndr {
                    let ith_prime = self.primes[i];
                    // Need to make this faster
                    result -= self.primes_less_than(bound / ith_prime);
                    if i < nprimes_below_3rdr {
                        let bi = self.primes_less_than(util::int_square_root(bound / ith_prime));
                        for j in i..bi {
                            let jth_prime = self.primes[j];
                            // p_i, p_j > bound^(1/4), so bound / (p_i * p_j) < bound ^ (1/2)
                            // Hence, this is a cheap lookup, and thus why we don't bother 
                            //  optimising this further...
                            result -= self.primes_less_than(bound / ith_prime / jth_prime) - j;
                        }
                    }
                }

                // Caching
                self.pi_cache.insert(bound, result);
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {        
    #[test]
    fn test_meissel_fn() {
        use crate::prime_count::PrimeCounter;
        let mut pc = PrimeCounter::new(10_000_000);
        assert_eq!(pc.meissel_fn(30, 8), 3);
        assert_eq!(pc.meissel_fn(100, 1), 50);
        assert_eq!(pc.meissel_fn(100, 25), 1);
        assert_eq!(pc.meissel_fn(10_000_000, 50), 1025747);
    }

    #[test]
    fn test_prime_pi() {
        use crate::prime_count::PrimeCounter;
        let mut pc = PrimeCounter::new(10_000);
        assert_eq!(pc.prime_pi(7), 4);
        assert_eq!(pc.prime_pi(100), 25);
        assert_eq!(pc.prime_pi(2143), 324);

        pc.update_limit(1_000_000_000);
        assert_eq!(pc.prime_pi(1_000_000), 78_498);
        assert_eq!(pc.prime_pi(1_000_000_000), 50_847_534);
        // pc.update_limit(1_000_000_000_000);
        // assert_eq!(pc.prime_pi(1_000_000_000_000), 37_607_912_018);
        // assert_eq!(pc.prime_pi(1_000_000_000_000_000), 29_844_570_422_669);
    }
}