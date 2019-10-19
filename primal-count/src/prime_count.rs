extern crate primal_sieve;
use std::iter::FromIterator;

use crate::util::{int_square_root, int_cubic_root, int_quartic_root};
use std::collections::HashMap;

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

/// The number of numbers less than m that are coprime to the first n prime numbers
/// Get recurrence m_fn(m, n) = m_fn(m, n - 1) - m_fn(m/p_n, n-1) by thinking about p_n, the nth prime
fn meissel_fn(m: usize, n: usize, prime_array: &Vec<usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return m;
    }
    match meissel_cache.get(&(m, n)).map(|entry| entry.clone()){
        Some(result) => result,
        None => {
            let value = meissel_fn(m, n-1, &prime_array, meissel_cache) 
                        - meissel_fn(m / prime_array[n-1], n-1, &prime_array, meissel_cache);
            meissel_cache.insert((m, n), value);
            return value;
        }
    }
}

/// Find the number of primes less than bound using the Meissel-Lehmer method
/// Leverages caching to speed up the recursive calls
fn primes_less_than(bound: usize, primes: &Vec<usize>, prime_cache: &mut HashMap<usize, usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // First check if it's in the cache already
    match prime_cache.get(&bound).map(|entry| entry.clone()){
        Some(value) => value,
        None => {
            // The meat of the function
            if bound < 2 {
                return 0;
            } else if bound <= primes[primes.len()-1] {
                let result = match primes.binary_search(&bound)
                {
                    Ok(idx) => idx+1,
                    Err(idx) => idx,
                };
                let result = result;
                prime_cache.insert(bound, result);
                return result;
            }
            
            let sqrt_bound = int_square_root(bound);

            let nprimes_below_4thr = primes_less_than(int_quartic_root(bound), primes, prime_cache, meissel_cache);
            let nprimes_below_3rdr = primes_less_than(int_cubic_root(bound), primes, prime_cache, meissel_cache);
            let nprimes_below_2ndr = primes_less_than(sqrt_bound, primes, prime_cache, meissel_cache);

            // Issues with underflow here if nprimes_below_2ndr + nprimes_below_4thr < 2
            // Dealt with by populating the offending (small) values in the cache at the top level
            let mut result = ((nprimes_below_2ndr + nprimes_below_4thr - 2) * (nprimes_below_2ndr - nprimes_below_4thr + 1)) / 2;
            result += meissel_fn(bound, nprimes_below_4thr, &primes, meissel_cache);

            for i in nprimes_below_4thr..nprimes_below_2ndr {
                let ith_prime = primes[i];
                result -= primes_less_than(bound / ith_prime, primes, prime_cache, meissel_cache);
                if i < nprimes_below_3rdr {
                    let bi = primes_less_than(int_square_root(bound / ith_prime), primes, prime_cache, meissel_cache);
                    for j in i..bi {
                        let jth_prime = primes[j];
                        result -= primes_less_than(bound / ith_prime / jth_prime, primes, prime_cache, meissel_cache) - j;
                    }
                }
            }

            // Caching
            prime_cache.insert(bound, result);
            return result;
        }
    }
}

/// Memoized combinatorial prime counting function
/// Basic idea here: https://en.wikipedia.org/wiki/Meissel%E2%80%93Lehmer_algorithm
/// The "Meissel Function" here is phi on that Wikipedia page
///
/// # Examples
///
pub struct PrimeCounter {
    limit: usize,
    primes: Vec<usize>,
    prime_cache: HashMap<usize, usize>,
    meissel_cache: HashMap<(usize, usize), usize>
}

impl PrimeCounter {
    /// Create a new PrimeCounter instance, which generates all the primes up to sqrt(limit)
    pub fn new(limit: usize) -> PrimeCounter {
        let mut prime_cache = HashMap::new();
        let primes = generate_primes(limit);

        // Insert primes <= 10 - this is mainly to deal with underflow issues later
        for n in 0..=10 {
            let nprimes = match n {
                2         => 1,
                3..=4     => 2,
                5..=6     => 3,
                7..=10    => 4,
                0..=1 | _ => 0,  // N.B. _ never hit
            };
            prime_cache.insert(n, nprimes);
        }
        let meissel_cache = HashMap::new();
        
        PrimeCounter {limit, primes, prime_cache, meissel_cache}
    }

    /// Bla
    pub fn update_limit(&mut self, limit: usize) {
        self.limit = limit;
        self.primes = generate_primes(int_square_root(limit));
    }
    
    /// Bla
    pub fn primes_below(&mut self, bound: usize) -> usize {
        primes_less_than(bound, &self.primes, &mut self.prime_cache, &mut self.meissel_cache)
    }

    /// Bla
    pub fn meissel_fn(&mut self, m: usize, n: usize) -> usize {
        meissel_fn(m, n, &self.primes, &mut self.meissel_cache)
    }
}

#[cfg(test)]
mod tests {        
    #[test]
    fn test_meissel_fn() {
        use prime_count::meissel_fn;
        use std::collections::HashMap;
        let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let mut meissel_cache = HashMap::new();
        assert_eq!(meissel_fn(30, 8, &prime_array, &mut meissel_cache), 3);
        assert_eq!(meissel_fn(100, 1, &prime_array, &mut meissel_cache), 50);
    }

    #[test]
    fn test_primes_below() {
        use crate::prime_count::PrimeCounter;
        let mut pc = PrimeCounter::new(10_000);
        assert_eq!(pc.primes_below(7), 4);
        assert_eq!(pc.primes_below(100), 25);
        assert_eq!(pc.primes_below(2143), 324);

        pc.update_limit(1_000_000_000);
        assert_eq!(pc.primes_below(1_000_000), 78_498);
        assert_eq!(pc.primes_below(1_000_000_000), 50_847_534);
        // assert_eq!(primes_below(1_000_000_000_000), 37_607_912_018);
        // assert_eq!(primes_below(1_000_000_000_000_000), 29_844_570_422_669);
    }
}