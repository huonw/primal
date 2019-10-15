use crate::util::integer_square_root;
use crate::util::integer_cubic_root;
use crate::util::integer_quartic_root;
use std::io;
use std::collections::HashMap;
use std::cmp::Ordering::*;

fn create_prime_array(bound: usize) -> Vec<usize> {
    // Returns an array of primes up to and including bound
    let mut vec = Vec::new();
    let max_value_to_check = integer_square_root(bound);
    let mut sieve = vec![true; bound + 1];
    sieve[0] = false;
    sieve[1] = false;
    for value in 2..bound+1 {
        if sieve[value] {
            // Add the prime to our vector
            vec.push(value);

            // Filter primes more if we're below the bound
            if value <= max_value_to_check {
                // Anything smaller than this already filtered
                let mut idx = value * value;
                while idx <= bound {
                    sieve[idx] = false;
                    idx += value;
                }
            }
        }
    }
    return vec;
}

fn meissel_fn_small(m: usize, n: usize, prime_array: &Vec<usize>) -> usize {
    // The number of numbers <= m that are coprime to the first n prime numbers.
    if n == 0 {
        return m;
    } else {
        return meissel_fn_small(m, n-1, &prime_array) - meissel_fn_small(m / prime_array[n-1], n-1, &prime_array);
    }
}

// def _meissel_function_large(self, m, n):
//     """The number of numbers <= m that are coprime to the first n prime numbers.
//     Run for larger values where repeating isn't going to happen often. Use for n > 1000 or so"""
//     m = int(m)
//     # if m <= 10000: return meis104[n][m]
//     if n == 0:
//         return m
//     result = self.meissel_memoize[n].get(m, None)
//     if result is None:
//         result = 0
//         primes_gen = (p for p in self.prime_array[:n:-1])
//         stacks = defaultdict(lambda: defaultdict(int))
//         stacks[n][m] = 1
//         for N in range(n, 0, -1):
//             prime_dividing = next(primes_gen)
//             for M in stacks[N]:
//                 # Marginal speed improvement?
//                 # if M <= 10000 and n<1229:
//                     # res += meis104[N][M]*stacks[N][M]
//                     # continue
//                 stacks[N-1][M] += stacks[N][M]
//                 stacks[N-1][M//prime_dividing] -= stacks[N][M]
//             del stacks[N]
//         for M in stacks[0]:
//             result += M*stacks[0][M]
//     return result
fn meissel_fn_large(m: usize, n: usize, prime_array: &Vec<usize>) -> usize {
    meissel_fn_small(m, n, &prime_array)
}

fn meissel_memoized(m: usize, n: usize, prime_array: &Vec<usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return m;
    }
    match meissel_cache.get(&(m, n)).map(|entry| entry.clone()){
        Some(result) => result,
        None => {
            let value = meissel_memoized(m, n-1, &prime_array, meissel_cache) 
                        - meissel_memoized(m / prime_array[n-1], n-1, &prime_array, meissel_cache);
            meissel_cache.insert((m, n), value);
            return value;
        }
    }
}

fn num_primes_less_than(bound: usize) -> isize {
    // Initialise prime array:
    if bound < 1000 {
        return create_prime_array(bound).len() as isize;
    }
    let sqrt_bound = integer_square_root(bound);
    let primes = create_prime_array(sqrt_bound);

    let a = num_primes_less_than(integer_quartic_root(bound));
    let c = num_primes_less_than(integer_cubic_root(bound));
    let b = num_primes_less_than(sqrt_bound);
    println!("a,b,c={},{},{}", a,b,c);
    let mut result = meissel_fn_small(bound, a as usize, &primes) as isize + ((b + a - 2) * (b - a + 1)) / 2;
    println!("result={}", result);

    for i in a..b {
        let ith_prime = primes[i as usize];
        result -= num_primes_less_than(bound / ith_prime);
        if i < c {
            let bi = num_primes_less_than(integer_square_root(bound / ith_prime));
            for j in i..bi {
                let jth_prime = primes[j as usize];
                result += j - num_primes_less_than(bound / ith_prime / jth_prime);
            }
        }
    }

    return result
}

fn num_primes_less_than_memoized(bound: usize, primes: &Vec<usize>, prime_cache: &mut HashMap<usize, usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // Initialise prime array:
    match prime_cache.get(&bound).map(|entry| entry.clone()){
        Some(value) => value,
        None => { // The meat of the function
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
            
            let sqrt_bound = integer_square_root(bound);

            let a = num_primes_less_than_memoized(integer_quartic_root(bound), primes, prime_cache, meissel_cache);
            let c = num_primes_less_than_memoized(integer_cubic_root(bound), primes, prime_cache, meissel_cache);
            let b = num_primes_less_than_memoized(sqrt_bound, primes, prime_cache, meissel_cache);

            // Issues with underflow here if b + a < 2
            let mut result = meissel_memoized(bound, a, &primes, meissel_cache) + ((b + a - 2) * (b - a + 1)) / 2;

            for i in a..b {
                let ith_prime = primes[i];
                result -= num_primes_less_than_memoized(bound / ith_prime, primes, prime_cache, meissel_cache);
                if i < c {
                    let bi = num_primes_less_than_memoized(integer_square_root(bound / ith_prime), primes, prime_cache, meissel_cache);
                    for j in i..bi {
                        let jth_prime = primes[j];
                        result -= (num_primes_less_than_memoized(bound / ith_prime / jth_prime, primes, prime_cache, meissel_cache) - j);
                    }
                }
            }

            // Caching and shit
            prime_cache.insert(bound, result);
            return result;
        }
    }
}

// Top level function
fn num_primes_less_than_main(bound: usize) -> usize {
    let primes = create_prime_array(integer_square_root(bound));
    let mut value_cache = HashMap::new();

    // Insert some basic primes - this is mainly to deal with underflow issues later
    value_cache.insert(0, 0);
    value_cache.insert(1, 0);
    value_cache.insert(2, 1);
    value_cache.insert(3, 2);
    value_cache.insert(4, 2);
    value_cache.insert(5, 3);
    value_cache.insert(6, 3);
    value_cache.insert(7, 4);
    value_cache.insert(8, 4);
    let mut meissel_cache = HashMap::new();
    println!("Primes initialised...");
    let value = num_primes_less_than_memoized(bound, &primes, &mut value_cache, &mut meissel_cache);
    println!("Value = {}", value_cache[&bound]);
    return value;
}



fn main() {
    // println!("How many primes below: ");
    // let mut input = String::new();

    // io::stdin().read_line(&mut input)
    //     .expect("Failed to read line");
    // let input: usize = input.trim().parse()
    //     .expect("Couldn't turn value into an integer...");

    // println!("There are {} primes below {}", create_prime_array(input).len(), input);
    // println!("There are {} primes below {}", num_primes_less_than_main(input), input);
    println!("There are {} primes", num_primes_less_than_main(10_000_000_000));


}

#[test]
fn test_int_sqrt() {
    assert_eq!(integer_square_root(0), 0);
    assert_eq!(integer_square_root(1), 1);
    assert_eq!(integer_square_root(16), 4);
    assert_eq!(integer_square_root(17), 4);
    assert_eq!(integer_square_root(24), 4);
    assert_eq!(integer_square_root(587 * 587 - 1), 586);
}

#[test]
fn test_int_cbrt() {
    assert_eq!(integer_cubic_root(0), 0);
    assert_eq!(integer_cubic_root(1), 1);
    assert_eq!(integer_cubic_root(26), 2);
    assert_eq!(integer_cubic_root(27), 3);
    assert_eq!(integer_cubic_root(28), 3);
    assert_eq!(integer_cubic_root(587 * 587 * 587 - 1), 586);
}

#[test]
fn test_int_quartic_root() {
    assert_eq!(integer_quartic_root(0), 0);
    assert_eq!(integer_quartic_root(1), 1);
    assert_eq!(integer_quartic_root(15), 1);
    assert_eq!(integer_quartic_root(16), 2);
    assert_eq!(integer_quartic_root(17), 2);
    assert_eq!(integer_quartic_root(587 * 587 * 587 * 587 - 1), 586);
    assert_eq!(integer_quartic_root(587 * 587 * 587 * 587 + 1), 587);
}

#[test]
fn test_prime_array() {
    let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
    assert_eq!(create_prime_array(19), prime_array);
    assert_eq!(create_prime_array(20), prime_array);
}

#[test]
fn test_meissel_fn_small() {
    let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
    assert_eq!(meissel_fn_small(30, 8, &prime_array), 3);
    assert_eq!(meissel_fn_small(100, 1, &prime_array), 50);
}

#[test]
fn test_meissel_fn_large() {
    let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
    assert_eq!(meissel_fn_large(30, 8, &prime_array), 3);
    assert_eq!(meissel_fn_large(100, 1, &prime_array), 50);
}


#[test]
fn test_meissel_fn_memoized() {
    let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let mut cache = HashMap::new();
    assert_eq!(meissel_memoized(30, 8, &prime_array, &mut cache), 3);
    assert_eq!(meissel_memoized(100, 1, &prime_array, &mut cache), 50);
}

#[test]
fn test_num_primes_less_than() {
    assert_eq!(num_primes_less_than(7), 4);
    assert_eq!(num_primes_less_than(100), 25);
    assert_eq!(num_primes_less_than(2143), 324);
    assert_eq!(num_primes_less_than(1_000_000), 78_498);
    // assert_eq!(num_primes_less_than(1_000_000_000), 50_847_534);
}

#[test]
fn test_num_primes_less_than_main() {
    assert_eq!(num_primes_less_than_main(7), 4);
    assert_eq!(num_primes_less_than_main(100), 25);
    assert_eq!(num_primes_less_than_main(2143), 324);
    assert_eq!(num_primes_less_than_main(1_000_000), 78_498);
    assert_eq!(num_primes_less_than_main(1_000_000_000), 50_847_534);
    // assert_eq!(num_primes_less_than_main(1_000_000_000_000), 37_607_912_018);
    // assert_eq!(num_primes_less_than_main(1_000_000_000_000_000), 29_844_570_422_669);
}